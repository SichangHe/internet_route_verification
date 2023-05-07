from sys import stderr
from typing import Iterable

from pyparsing import ParseException, ParserElement

from .afi import afi_set_intersection_difference, merge_afi_dict
from .lex import action, afi, as_expr, mp_filter, mp_peering


def lex_with(lexer: ParserElement, string: str) -> dict | None:
    try:
        return lexer.parse_string(string).as_dict()
    except ParseException as err:
        print(f"{err} parsing `{string}`.", file=stderr)


def clean_action(
    actions_lexed: Iterable[dict[str, dict | list]]
) -> dict[str, str | list[str] | list[dict[str, str | list[str]]]]:
    """Clean up a stream of <action>s
    -> {[<assignee1>...]: str | list[str], [community]: list[{
        [method]: str, args: list[str]
    }], [<rp-attribute1>...]: list[{method: str, args: list[str]}]}"""
    cleaned = {}
    for action_lexed in actions_lexed:
        if assignment := action_lexed.get("assignment"):
            assert isinstance(assignment, dict)
            cleaned[assignment["assignee"]] = (
                assigned
                if (assigned := assignment.get("assigned"))
                else assignment["assigned-set"]
            )
        elif community := action_lexed.get("community"):
            assert isinstance(community, dict)
            community_entry = cleaned.get("community", [])
            community_entry.append(community)
            cleaned["community"] = community_entry
        elif add_community := action_lexed.get("add-community"):
            assert isinstance(add_community, list)
            community_entry = cleaned.get("community", [])
            community_entry.append({"method": "=", "args": add_community})
            cleaned["community"] = community_entry
        elif method_call := action_lexed.get("method-call"):
            assert isinstance(method_call, dict)
            rp_attribute = method_call.pop("rp-attribute")
            rp_entry = cleaned.get(rp_attribute, [])
            rp_entry.append(method_call)
            cleaned[rp_attribute] = rp_entry
    return cleaned


def clean_mp_filter_base(lexed: dict) -> dict[str, dict | list] | list[str | list[str]]:
    """community -> {community: {[method]: str, args: list[str]}}
    policy-filter -> list[str | list[str]]
    mp_filter -> ..."""
    if "community" in lexed:
        return lexed
    if policy_filter := lexed.get("policy-filter"):
        return policy_filter
    return clean_mp_filter(lexed)


def clean_mp_filter(
    lexed: dict,
) -> dict[str, dict | list] | list[str | list[str]]:
    """-> {(and | or: {left, right}) | not}
    | {community: {[method]: str, args: list[str]}}
    | list[str | list[str]]"""
    if inner := lexed.get("and"):
        return {
            "and": {
                "left": clean_mp_filter(inner["left"]),
                "right": clean_mp_filter(inner["right"]),
            }
        }
    if inner := lexed.get("or"):
        return {
            "or": {
                "left": clean_mp_filter(inner["left"]),
                "right": clean_mp_filter(inner["right"]),
            }
        }
    if inner := lexed.get("not"):
        return {"not": clean_mp_filter(inner)}
    return clean_mp_filter_base(lexed)


def clean_as_expr(lexed: dict) -> str | dict:
    """ "-> str | {and | or | except: {left, right}}"""
    if inner := lexed.get("field"):
        return inner
    if inner := lexed.get("and"):
        return {
            "and": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }
    if inner := lexed.get("or"):
        return {
            "or": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }
    if inner := lexed.get("except"):
        return {
            "except": {
                "left": clean_as_expr(inner["left"]),
                "right": clean_as_expr(inner["right"]),
            }
        }
    raise ValueError(f"Illegal keys: {lexed}")


def clean_mp_peering(lexed: dict) -> str | dict[str, str | dict] | None:
    """-> str | {
        as_expr: str | {and | or | except: {left, right}},
        [router_expr1]: str | {and | or | except: {left, right}},
        [router_expr2]: str | {and | or | except: {left, right}}
    }"""
    if peering_set := lexed.get("peering-set-name"):
        return peering_set
    as_expr_raw = " ".join(lexed["as-expression"])
    if expr := lex_with(as_expr, as_expr_raw):
        result = {"as_expr": clean_as_expr(expr)}
    else:
        return
    if (expr1 := lexed.get("mp-router-expression-1")) and (
        expr := lex_with(as_expr, " ".join(expr1))
    ):
        result["router_expr1"] = clean_as_expr(expr)
    if (expr2 := lexed.get("mp-router-expression-2")) and (
        expr := lex_with(as_expr, " ".join(expr2))
    ):
        result["router_expr2"] = clean_as_expr(expr)
    return result


def parse_mp_peering(mp_peering_raw: list[str]):
    """-> str | {
        as_expr: str | {and | or | except: {left, right}},
        [router_expr1]: str | {and | or | except: {left, right}},
        [router_expr2]: str | {and | or | except: {left, right}}
    }"""
    if lexed := lex_with(mp_peering, " ".join(mp_peering_raw)):
        return clean_mp_peering(lexed)


def parse_import_factor(import_factor_raw: dict) -> dict[str, list | dict] | None:
    """-> {
        mp_peerings: list[{
            mp_peering: str | {as_expr, [router_expr1], [router_expr2]}
            actions: {[<assignee1>...]: str | list[str], [community]: list[{
                [method]: str, args: list[str]
            }], [<rp-attribute1>...]: list[{method: str, args: list[str]}]}
        }],
        mp_filter: {(and | or: {left, right}) | not}
            | {community: {[method]: str, args: list[str]}}
            | list[str | list[str]]
    }"""
    import_factor: dict[str, list | dict] = {"mp_peerings": []}
    if filter := lex_with(mp_filter, import_factor_raw["mp-filter"]):
        import_factor["mp_filter"] = clean_mp_filter(filter)
    else:
        return
    for peering_raw in import_factor_raw["mp-peerings"]:
        peering = {}
        if peer := parse_mp_peering(peering_raw["mp-peering"]):
            peering["mp_peering"] = peer
        else:
            continue
        if action_raws := peering_raw.get("actions"):
            peering["actions"] = clean_action(
                action_lexed
                for action_raw in action_raws
                if (action_lexed := lex_with(action, action_raw))
            )
        import_factor["mp_peerings"].append(peering)  # type: ignore
    return import_factor


def apply_except(left: dict[str, list | dict], right: dict):
    """The resulting policy set contains the policies of the right hand side
    but their filters are modified to only include the routes also matched by
    the left hand side.
    The policies of the left hand side are included afterwards and
    their filters are modified to exclude the routes matched by the right
    hand side."""
    first = {
        "mp_peerings": right["mp_peerings"],
        "mp_filter": {"and": {"left": left["mp_filter"], "right": right["mp_filter"]}},
    }
    second = {
        "mp_peerings": left["mp_peerings"],
        "mp_filter": {
            "and": {"left": left["mp_filter"], "right": {"not": right["mp_filter"]}}
        },
    }
    return [first, second]


def parse_import_term(
    lexed: dict,
) -> list[dict[str, list | dict]] | None:
    if import_factors := lexed.get("import-factors"):
        parsed = []
        for import_factor_raw in import_factors:
            if import_factor := parse_import_factor(import_factor_raw):
                parsed.append(import_factor)
        return parsed

    if "mp-peerings" in lexed and "mp-filter" in lexed:
        if import_factor := parse_import_factor(lexed):
            return [import_factor]


def parse_import_expression_except(
    lexed: dict, afi_entries: set[tuple[str, str]]
) -> list[tuple[set[tuple[str, str]], list[dict]]]:
    result = []
    right = parse_afi_import_expression(lexed["right"], afi_entries)
    if lefts := parse_import_term(lexed["left"]):
        assert len(lefts) == 1
        left = lefts[0]
    else:
        print(f"Skipping because import-term not parsed: {lexed}", file=stderr)
        return []
    for right_afis, parsed_list in right:
        intersection, difference = afi_set_intersection_difference(
            afi_entries, right_afis
        )
        if len(difference) > 0:
            # This part of the EXCEPT clause is ignored.
            result.append((difference, [left]))

        """The address family may be specified in subsequent refine or except
        policy expressions and is valid only within the policy expression
        that contains it."""
        if len(intersection) > 0:
            applied_list = [
                applied
                for parsed in parsed_list
                for applied in apply_except(left, parsed)
            ]
            result.append((intersection, applied_list))

    return result


def parse_afi_import_expression(
    afi_import_expression: dict, afi_entries: set[tuple[str, str]]
) -> list[tuple[set[tuple[str, str]], list[dict]]]:
    """-> list[tuple[afi_entries, parsed]]"""
    if afi_list := afi_import_expression.get("afi-list"):
        afi_entries = merge_afi_dict(
            afi_item for item in afi_list if (afi_item := lex_with(afi, item))
        )

    if import_term := parse_import_term(afi_import_expression):
        return [(afi_entries, import_term)]

    if except_expr := afi_import_expression.get("except"):
        return parse_import_expression_except(except_expr, afi_entries)

    if "refine" in afi_import_expression:
        # TODO: Handle REFINE logic.
        print(f"Skipping complex logic in {afi_import_expression}", file=stderr)

    return []


def import_export(lexed: dict, result: dict[str, dict[str, list]]):
    """Parse lexed <mp-import> or <mp-export>."""
    if protocol_1 := lexed.get("protocol-1"):
        print(f"Ignoring protocol-1: {protocol_1}.", file=stderr)
    if protocol_2 := lexed.get("protocol-2"):
        print(f"Ignoring protocol-2: {protocol_2}.", file=stderr)

    parsed_list = parse_afi_import_expression(lexed, set([("any", "any")]))
    for afi_entries, parsed in parsed_list:
        for version, cast in afi_entries:
            version_entry = result.get(version, {})
            cast_entry = version_entry.get(cast, [])
            cast_entry.extend(parsed)
            version_entry[cast] = cast_entry
            result[version] = version_entry
    return result

from typing import Iterable

from pyparsing import ParseException, ParserElement

from .afi import afi_set_intersection_difference, merge_afi_dict
from .lex import action, afi, as_expr, mp_filter, mp_peering


def lex_with(lexer: ParserElement, string: str) -> dict:
    try:
        return lexer.parse_string(string, parse_all=True).as_dict()
    except ParseException:
        raise ValueError(f"ParseException: Parsing `{string}`")


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
    regex -> {regex: str}
    policy-filter -> {path_attr: str}
    address-prefix-set -> {addr_prefix_set: list[str]}"""
    if "community" in lexed or "regex" in lexed:
        return lexed
    if policy_filter := lexed.get("filter"):
        return {"path_attr": policy_filter}
    if (addr_prefix_set := lexed.get("address-prefix-set")) is not None:
        return {"addr_prefix_set": addr_prefix_set}
    raise ValueError(f"{lexed} is not in a valid <mp-filter> base form.")


def clean_mp_filter(
    lexed: dict,
) -> dict[str, dict | list] | list[str | list[str]]:
    """-> {
        (and | or: {left, right}) | not | group
        | community: {[method]: str, args: list[str]}
        | regex: str | path_attr: str | addr_prefix_set: list[str]
    }"""
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
    if inner := lexed.get("group"):
        return {"group": clean_mp_filter(inner)}
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
    if inner := lexed.get("group"):
        return {"group": clean_as_expr(inner)}
    raise ValueError(f"Illegal keys: {lexed}")


def clean_mp_peering(lexed: dict) -> dict[str, str | dict]:
    """-> {
        as_expr: str | {and | or | except: {left, right} | group: {...}},
        [router_expr1]: str | {and | or | except: {left, right} | group: {...}},
        [router_expr2]: str | {and | or | except: {left, right} | group: {...}}
    }"""
    as_expr_raw = " ".join(lexed["as-expression"])
    expr = lex_with(as_expr, as_expr_raw)
    result = {"as_expr": clean_as_expr(expr)}
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
    """-> {
        as_expr: str | {and | or | except: {left, right} | group: {...}},
        [router_expr1]: str | {and | or | except: {left, right} | group: {...}},
        [router_expr2]: str | {and | or | except: {left, right} | group: {...}}
    }"""
    lexed = lex_with(mp_peering, " ".join(mp_peering_raw))
    return clean_mp_peering(lexed)


def parse_import_factor(import_factor_raw: dict) -> dict[str, list | dict]:
    """-> {
        mp_peerings: list[{
            mp_peering: {as_expr, [router_expr1], [router_expr2]}
            actions: {[<assignee1>...]: str | list[str], [community]: list[{
                [method]: str, args: list[str]
            }], [<rp-attribute1>...]: list[{method: str, args: list[str]}]}
        }],
        [mp_filter]: {(and | or: {left, right}) | not | group}
            | {community: {[method]: str, args: list[str]}}
            | list[str | list[str]]
    }"""
    import_factor: dict[str, list | dict] = {"mp_peerings": []}
    if filter_raw := import_factor_raw.get("mp-filter"):
        filter = lex_with(mp_filter, filter_raw)
        import_factor["mp_filter"] = clean_mp_filter(filter)
    for peering_raw in import_factor_raw["mp-peerings"]:
        peering = {}
        peer = parse_mp_peering(peering_raw["mp-peering"])
        peering["mp_peering"] = peer
        if action_raws := peering_raw.get("actions"):
            peering["actions"] = clean_action(
                lex_with(action, action_raw) for action_raw in action_raws
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
            import_factor = parse_import_factor(import_factor_raw)
            parsed.append(import_factor)
        return parsed

    if "mp-peerings" in lexed:
        import_factor = parse_import_factor(lexed)
        return [import_factor]

    return None


def parse_import_expression_except(
    lexed: dict, afi_entries: set[tuple[str, str]]
) -> list[tuple[set[tuple[str, str]], list[dict]]]:
    result = []
    right = parse_afi_import_expression(lexed["right"], afi_entries)
    if lefts := parse_import_term(lexed["left"]):
        assert len(lefts) == 1
        left = lefts[0]
    else:
        raise ValueError(f"Import-term not parsed: {lexed}")
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


def try_get_and(key: str, left: dict[str, dict], right: dict[str, dict]) -> dict | None:
    if left_value := left.get(key):
        if right_value := right.get(key):
            return {"and": {"left": left_value, "right": right_value}}
        else:
            return left_value
    elif right_value := right.get(key):
        return right_value
    return None


def try_get_merge(
    key: str, left: dict[str, dict], right: dict[str, dict]
) -> dict | None:
    if left_value := left.get(key):
        if right_value := right.get(key):
            left_value_copy = left_value.copy()
            left_value_copy.update(right_value)
            return left_value_copy
        else:
            return left_value
    elif right_value := right.get(key):
        return right_value
    return None


def apply_refine(left: dict[str, list | dict], right: dict) -> dict[str, dict | list]:
    left_peerings = left["mp_peerings"]
    assert len(left_peerings) == 1
    left_peering_action = left_peerings[0]
    left_peering = left_peering_action["mp_peering"]

    right_peerings = right["mp_peerings"]
    # TODO: Deal with multiple <mp-peering>s.
    if len(right_peerings) > 1:
        raise ValueError(
            f"Skip: Skipping REFINE expression with multiple <mp-peering>s: {right}."
        )
    right_peering_action = right_peerings[0]
    right_peering = right_peering_action["mp_peering"]
    combined_peering = {
        "as_expr": {
            "and": {
                "left": left_peering["as_expr"],
                "right": right_peering["as_expr"],
            }
        }
    }

    if combined_router_expr1 := try_get_and(
        "router_expr1", left_peering, right_peering
    ):
        combined_peering["router_expr1"] = combined_router_expr1
    if combined_router_expr2 := try_get_and(
        "router_expr2", left_peering, right_peering
    ):
        combined_peering["router_expr2"] = combined_router_expr2

    combined_peering_action = {"mp_peering": combined_peering}

    if combined_actions := try_get_merge(
        "actions", left_peering_action, right_peering_action
    ):
        combined_peering_action["actions"] = combined_actions

    return {
        "mp_peerings": [combined_peering_action],
        "mp_filter": {"and": {"left": left["mp_filter"], "right": right["mp_filter"]}},
    }


def parse_import_expression_refine(
    lexed: dict, afi_entries: set[tuple[str, str]]
) -> list[tuple[set[tuple[str, str]], list[dict]]]:
    """<https://www.rfc-editor.org/rfc/rfc2622#page-36>"""
    result = []
    right = parse_afi_import_expression(lexed["right"], afi_entries)
    lefts = parse_import_term(lexed["left"])
    if lefts is None:
        raise ValueError(f"Import-term not parsed: {lexed}")
    for right_afis, parsed_list in right:
        for left in lefts:
            intersection, difference = afi_set_intersection_difference(
                afi_entries, right_afis
            )
            if len(difference) > 0:
                # This part of the REFINE clause is ignored.
                result.append((difference, [left]))

            """The address family may be specified in subsequent refine or except
            policy expressions and is valid only within the policy expression
            that contains it."""
            if len(intersection) > 0:
                applied_list = [apply_refine(left, parsed) for parsed in parsed_list]
                result.append((intersection, applied_list))

    return result


def parse_afi_import_expression(
    afi_import_expression: dict, afi_entries: set[tuple[str, str]]
) -> list[tuple[set[tuple[str, str]], list[dict]]]:
    """-> list[tuple[afi_entries, parsed]]"""
    if afi_list := afi_import_expression.get("afi-list"):
        afi_entries = merge_afi_dict(lex_with(afi, item) for item in afi_list)

    if import_term := parse_import_term(afi_import_expression):
        return [(afi_entries, import_term)]

    if except_expr := afi_import_expression.get("except"):
        return parse_import_expression_except(except_expr, afi_entries)

    if refine_expr := afi_import_expression.get("refine"):
        return parse_import_expression_refine(refine_expr, afi_entries)

    return []


def import_export(lexed: dict, result: dict[str, dict[str, list]], is_mp: bool = False):
    """Parse lexed <mp-import> or <mp-export>."""
    if is_mp:
        if protocol_1 := lexed.get("protocol-1"):
            print(f"Ignore: Ignoring protocol-1: {protocol_1}.")
        if protocol_2 := lexed.get("protocol-2"):
            print(f"Ignore: Ignoring protocol-2: {protocol_2}.")

    afi_entries = set([("any", "any") if is_mp else ("ipv4", "unicast")])
    parsed_list = parse_afi_import_expression(lexed, afi_entries)
    for afi_entries, parsed in parsed_list:
        for version, cast in afi_entries:
            version_entry = result.get(version, {})
            cast_entry = version_entry.get(cast, [])
            cast_entry.extend(parsed)
            version_entry[cast] = cast_entry
            result[version] = version_entry
    return result

from sys import stderr
from typing import Generator, Iterable

from pyparsing import ParseException, ParserElement

from .lex import action, afi, mp_filter, mp_peering


def lex_with(lexer: ParserElement, string: str) -> dict | None:
    try:
        return lexer.parse_string(string).as_dict()
    except ParseException as err:
        print(f"{err} parsing `{string}`.", file=stderr)


def import_factors_in_flat(afi_import_expression: dict) -> Generator[dict, None, None]:
    """Extract <import-factor>s from <afi-import-expression>, ignoring nesting.
    -> mp-peerings: list[{mp-peering, [actions]}], mp-filter: str"""
    if import_factors := afi_import_expression.get("import-factors"):
        for import_factor in import_factors:
            yield import_factor
    elif (
        "mp-peerings" in afi_import_expression and "mp-filter" in afi_import_expression
    ):
        yield afi_import_expression


def afi_import_expressions(lexed: dict) -> Generator[dict, None, None]:
    """Extract flattened <afi-import-expression>s in a lexed <mp-import> or
    <afi-import-expression>.
    -> {[import-term: {
            import-factors: list[{mp-peerings, mp-filter}]
            | (mp-peerings, mp-filter)
        }, logic: str],
    [afi-list]: list[str], (
        import-expression | import-factors: list[{mp-peerings, mp-filter}]
        | (mp-peerings, mp-filter)
    )}"""
    if import_expr := lexed.get("import-expression"):
        yield lexed
        for afi_import_expression in afi_import_expressions(import_expr):
            yield afi_import_expression
    if "import-factors" in lexed or ("mp-peerings" in lexed and "mp-filter" in lexed):
        yield lexed


def merge_afi(afis: Iterable[dict[str, str]]) -> list[tuple[str, str]]:
    afi_sets: dict[str, set[str]] = {}
    for afi_item in afis:
        version = afi_item["version"]
        cast = afi_item.get("cast", "any")
        entry = afi_sets.get(version, set())
        entry.add(cast)
        afi_sets[version] = entry
    afi_map: dict[str, str] = {}
    for key, afi_set in afi_sets.items():
        if "any" in afi_set or ("unicast" in afi_set and "multicast" in afi_set):
            afi_map[key] = "any"
        else:
            assert len(afi_set) == 1
            afi_map[key] = afi_set.pop()
    if (v4 := afi_map.get("ipv4")) and (v6 := afi_map.get("ipv6")) and (v4 == v6):
        return [("any", v4)]
    return [(key, value) for key, value in afi_map.items()]


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
                if (assigned := assignment["assigned"])
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


def clean_mp_filter_base(lexed: dict):
    """community -> {community: {[method]: str, args: list[str]}}
    policy-filter -> list[str | list[str]]
    mp_filter -> ..."""
    if "community" in lexed:
        return lexed
    if policy_filter := lexed.get("policy-filter"):
        return policy_filter
    return clean_mp_filter(lexed)


def clean_mp_filter(lexed: dict):
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


def import_export(lexed: dict, result: dict[str, dict[str, list]]):
    if protocol_1 := lexed.get("protocol-1"):
        print(f"Ignoring protocol-1: {protocol_1}.", file=stderr)
    if protocol_2 := lexed.get("protocol-2"):
        print(f"Ignoring protocol-2: {protocol_2}.", file=stderr)

    for afi_import_expression in afi_import_expressions(lexed):
        afi_entries = [("any", "any")]
        if afi_list := afi_import_expression.get("afi-list"):
            afi_entries = merge_afi(
                afi_item for item in afi_list if (afi_item := lex_with(afi, item))
            )
        if "import-term" in afi_import_expression:
            # TODO: Handle EXCEPT and REFINE logic.
            print(f"Skipping complex logic in {afi_import_expression}", file=stderr)
        for import_factor_raw in import_factors_in_flat(afi_import_expression):
            import_factor: dict[str, list | dict] = {"mp_peerings": []}
            if filter := lex_with(mp_filter, import_factor_raw["mp-filter"]):
                import_factor["mp_filter"] = clean_mp_filter(filter)
            else:
                continue
            for peering_raw in import_factor_raw["mp-peerings"]:
                peering = {}
                if peer := lex_with(mp_peering, "".join(peering_raw["mp-peering"])):
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
            for version, cast in afi_entries:
                version_entry = result.get(version, {})
                cast_entry = version_entry.get(cast, [])
                cast_entry.append(import_factor)
                version_entry[cast] = cast_entry
                result[version] = version_entry
    return result

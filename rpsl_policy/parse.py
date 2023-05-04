from sys import stderr
from typing import Generator, Iterable

from pyparsing import ParseException, ParserElement

from .lex import afi


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
    -> {[afi-list]: list[str], (
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


def import_export(lexed: dict):
    if protocol_1 := lexed.get("protocol-1"):
        print(f"Ignoring protocol-1: {protocol_1}.", file=stderr)
    if protocol_2 := lexed.get("protocol-2"):
        print(f"Ignoring protocol-2: {protocol_2}.", file=stderr)

    result = {}

    for afi_import_expression in afi_import_expressions(lexed):
        afi_entries = [("any", "any")]
        if afi_list := afi_import_expression.get("afi-list"):
            afi_entries = merge_afi(
                afi_item for item in afi_list if (afi_item := lex_with(afi, item))
            )
        # TODO: Parse <import-expression>s.

    return result

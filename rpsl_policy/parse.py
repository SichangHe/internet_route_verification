from typing import Generator


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


def import_export(lexed: dict):
    # TODO: Implement.
    pass

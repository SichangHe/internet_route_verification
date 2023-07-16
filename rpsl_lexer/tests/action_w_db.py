from io import TextIOWrapper
from random import choices
from typing import Generator

from pyparsing import ParseException

from ..lex import action, mp_import
from ..lines import io_wrapper_lines, lines_continued
from .mp_peering_w_db import get_import_factors


def parse_action(expr: str, verbose: bool = False):
    if verbose:
        success, results = action.run_tests(expr, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not action.matches(expr):
        # Match failed.
        parse_action(expr, True)


def action_raws_in_import_factor(
    import_factor: dict,
) -> Generator[str, None, None]:
    peerings = import_factor["mp-peerings"]
    for mp_peering_raw in peerings:
        if action_raws := mp_peering_raw.get("actions"):
            for action_raw in action_raws:
                yield action_raw


def parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    try:
        result = mp_import.parse_string(value).as_dict()
    except ParseException:
        return
    import_factors = get_import_factors(result)
    for import_factor in import_factors:
        for action_raw in action_raws_in_import_factor(import_factor):
            parse_action(action_raw, verbose)


def parse_statement(statement: str, verbose: bool = False):
    if ":" not in statement:
        return 0, 0

    if statement.startswith("mp-import"):
        parse_mp_import(statement, verbose)
        return 1, 0
    elif statement.startswith("mp-export"):
        parse_mp_import(statement, verbose)
        return 0, 1

    return 0, 0


def read_db_test_parser(db: TextIOWrapper):
    line: str = ""
    n_mp_import = 0
    n_mp_export = 0
    db_lines = io_wrapper_lines(db)
    for line in lines_continued(db_lines):
        # 1% chance verbose.
        verbose = choices((True, False), (1, 99))[0]
        i, e = parse_statement(line, verbose)
        n_mp_import += i
        n_mp_export += e
    print(f"Read {n_mp_import} mp-imports, {n_mp_export} mp-exports.")


def main():
    with open("data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

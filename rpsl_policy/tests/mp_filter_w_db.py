from io import TextIOWrapper
from random import choices

from ..lex import mp_filter, mp_import
from ..lines import io_wrapper_lines, lines_continued
from .mp_peering_w_db import get_import_factors


def parse_mp_filter(expr: str, verbose: bool = False):
    if verbose:
        success, results = mp_filter.run_tests(expr, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not mp_filter.matches(expr):
        # Match failed.
        parse_mp_filter(expr, True)


def parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    result = mp_import.parse_string(value).as_dict()
    import_factors = get_import_factors(result)
    for import_factor in import_factors:
        mp_filter_raw = import_factor["mp-filter"]
        parse_mp_filter(mp_filter_raw, verbose and (" " in mp_filter_raw))


def parse_statement(statement: str, verbose: bool = False):
    if ":" not in statement or not statement.startswith("mp-import"):
        return 0
    parse_mp_import(statement, verbose)
    return 1


def read_db_test_parser(db: TextIOWrapper):
    line: str = ""
    n_mp_import = 0
    db_lines = io_wrapper_lines(db)
    for line in lines_continued(db_lines):
        # 1% chance verbose.
        verbose = choices((True, False), (1, 99))[0]
        n_mp_import += parse_statement(line, verbose)
    print(f"Read {n_mp_import} mp-imports.")


def main():
    with open("data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

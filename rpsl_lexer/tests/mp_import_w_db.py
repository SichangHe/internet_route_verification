from io import TextIOWrapper
from random import choices

from ..lex import mp_import
from ..lines import io_wrapper_lines, lines_continued


def parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    if verbose:
        success, results = mp_import.run_tests(value, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not mp_import.matches(value):
        # Match failed.
        parse_mp_import(line, True)


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

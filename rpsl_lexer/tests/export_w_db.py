from io import TextIOWrapper
from random import choices

from rpsl_lexer.lines import io_wrapper_lines, lines_continued
from .mp_import_w_db import parse_mp_import


def parse_statement(statement: str, verbose: bool = False):
    if not statement.startswith("export:"):
        return 0
    parse_mp_import(statement, verbose)
    return 1


def read_db_test_parser(db: TextIOWrapper):
    line: str = ""
    n_export = 0
    db_lines = io_wrapper_lines(db)
    for line in lines_continued(db_lines):
        # 1% chance verbose.
        verbose = choices((True, False), (1, 99))[0]
        n_export += parse_statement(line, verbose)
    print(f"Read {n_export} mp-exports.")


def main():
    with open("data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

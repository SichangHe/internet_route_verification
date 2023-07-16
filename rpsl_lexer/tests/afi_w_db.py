from io import TextIOWrapper
from random import choices

from pyparsing import ParseException

from ..lex import afi, mp_import
from ..lines import io_wrapper_lines, lines_continued
from ..parse import afi_import_expressions


def parse_afi(expr: str, verbose: bool = False):
    if verbose:
        success, results = afi.run_tests(expr, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not afi.matches(expr):
        # Match failed.
        parse_afi(expr, True)


def parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    try:
        result = mp_import.parse_string(value).as_dict()
    except ParseException:
        return
    for afi_import_expression in afi_import_expressions(result):
        if afi_list := afi_import_expression.get("afi-list"):
            for afi_item in afi_list:
                parse_afi(afi_item, verbose)


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

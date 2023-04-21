from io import TextIOWrapper
from random import choices

from pytest import mark

from ..lex import mp_import


@mark.skip
def test_parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    if verbose:
        success, results = mp_import.run_tests(value, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not mp_import.matches(value):
        # Match failed.
        test_parse_mp_import(line, True)


@mark.skip
def test_parse_statement(statement: str, verbose: bool = False):
    if ":" not in statement or not statement.startswith("mp-import"):
        return 0
    test_parse_mp_import(statement, verbose)
    return 1


def read_db_test_parser(db: TextIOWrapper):
    continuation_chars = (" ", "+", "\t")
    last_line: str = ""
    line: str = ""
    n_mp_import = 0
    while line := db.readline():
        # Remove comments.
        line = line.split("#", maxsplit=1)[0]

        # Handle continuation lines.
        if line.startswith(continuation_chars):
            last_line += " " + line[1:].strip()
            continue

        # Test complete statement.
        if last_line:
            # 1% chance verbose.
            verbose = choices((True, False), (1, 99))[0]
            n_mp_import += test_parse_statement(last_line, verbose)

        last_line = line.strip()
    print(f"Read {n_mp_import} mp-imports.")


def main():
    with open("data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

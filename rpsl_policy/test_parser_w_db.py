from io import TextIOWrapper
from random import choices

from parse import lex


def test_parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    if verbose:
        success, results = lex.run_tests(value, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not lex.matches(value):
        # Match failed.
        test_parse_mp_import(line, True)


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
    while n_mp_import < 100000:
        # Read 1 line.
        line = db.readline()

        if not line:
            continue

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


def main():
    with open("../data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

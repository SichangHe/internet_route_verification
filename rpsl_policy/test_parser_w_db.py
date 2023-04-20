from io import TextIOWrapper

from parse import lex


def test_parse_line(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    if verbose:
        success, results = lex.run_tests(value, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not lex.matches(value):
        # Match failed.
        test_parse_line(line, True)


def read_db_test_parser(db: TextIOWrapper):
    line: str
    n_mp_import = 0
    while n_mp_import < 10000:
        try:
            # TODO: Deal with continuation lines.
            line = db.readline()
            if ":" in line and line.startswith("mp-import"):
                verbose = n_mp_import % 10 == 0
                test_parse_line(line, verbose)
                n_mp_import += 1
        except UnicodeDecodeError as err:
            print(err)


def main():
    with open("../data/ripe.db", "r") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

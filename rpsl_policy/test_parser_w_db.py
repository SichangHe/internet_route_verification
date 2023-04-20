from io import TextIOWrapper

from parse import lex


def test_parse_line(line: str):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    success, results = lex.run_tests(value, full_dump=False)
    if success:
        print(results[0][1].as_dict())  # type: ignore


def read_db_test_parser(db: TextIOWrapper):
    line: str
    n_mp_import = 0
    while n_mp_import < 1000:
        try:
            line = db.readline()
            if ":" in line and line.startswith("mp-import"):
                test_parse_line(line)
                n_mp_import += 1
        except UnicodeDecodeError as err:
            print(err)


def main():
    with open("../data/ripe.db", "r") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

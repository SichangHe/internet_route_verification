from io import TextIOWrapper

from pyparsing import ParseException

from ..dump import red
from ..lex import mp_import
from ..lines import io_wrapper_lines, lines_continued

mp_imports = []
mp_exports = []
imports = []
exports = []


def parse_mp_import(line: str):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    return mp_import.parse_string(value)


def parse_statement(statement: str):
    try:
        if statement.startswith("mp-import:"):
            mp_imports.append(parse_mp_import(statement))
        if statement.startswith("mp-export:"):
            mp_exports.append(parse_mp_import(statement))
        if statement.startswith("import:"):
            imports.append(parse_mp_import(statement))
        if statement.startswith("export:"):
            exports.append(parse_mp_import(statement))
    except ParseException:
        tag = red("[parse_statement]")
        print(f"{tag} ParseException parsing {statement}")


def read_db_test_parser(db: TextIOWrapper):
    line: str = ""
    db_lines = io_wrapper_lines(db)
    for line in lines_continued(db_lines):
        parse_statement(line)


def main():
    with open("data/ripe.db", "r", encoding="latin-1") as db:
        read_db_test_parser(db)


if __name__ == "__main__":
    main()

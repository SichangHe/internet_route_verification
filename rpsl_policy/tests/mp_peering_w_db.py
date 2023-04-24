from io import TextIOWrapper
from random import choices
from typing import Generator

from ..lex import mp_import, mp_peering
from ..lines import io_wrapper_lines, lines_continued


def parse_mp_peering(expr: str, verbose: bool = False):
    if verbose:
        success, results = mp_peering.run_tests(expr, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore
    elif not mp_peering.matches(expr):
        # Match failed.
        parse_mp_peering(expr, True)


def get_import_factors(parsed: dict) -> Generator[dict, None, None]:
    if import_factors := parsed.get("import-factors"):
        for import_factor in import_factors:
            yield import_factor
    elif parsed.get("from"):
        yield parsed


def mp_peering_raws_in_import_factor(
    import_factor: dict,
) -> Generator[str, None, None]:
    frm = import_factor["from"]
    for mp_peering_raw in frm:
        yield " ".join(
            # list[str]
            mp_peering_raw["mp-peering"]
        )


def parse_mp_import(line: str, verbose: bool = False):
    _, value = line.split(":", maxsplit=1)
    value = value.strip()
    result = mp_import.parse_string(value).as_dict()
    import_factors = get_import_factors(result)
    for import_factor in import_factors:
        for mp_peering_raw in mp_peering_raws_in_import_factor(import_factor):
            parse_mp_peering(mp_peering_raw, verbose and (" " in mp_peering_raw))


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

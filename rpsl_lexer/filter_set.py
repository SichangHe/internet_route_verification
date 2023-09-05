import sys

from .dump import red
from .lex import mp_filter
from .lines import expressions
from .parse import clean_mp_filter, lex_with
from .piped import stdin_lines, write_obj
from .rpsl_object import FilterSet


def parse_filter_set():
    filters = []
    for key, expr in expressions(stdin_lines()):
        if key == "filter" or key == "mp-filter":
            try:
                lexed = lex_with(mp_filter, expr)
                parsed = clean_mp_filter(lexed)
                filters.append(parsed)
            except Exception as err:
                tag = red("[parse_filter_set]")
                print(
                    f"{tag} {err} parsing `{expr}`.",
                    file=sys.stderr,
                )
    return FilterSet("", "", filters).__dict__


def main():
    print("Launching filter_set lexer.", file=sys.stderr)

    while True:
        filter_set = parse_filter_set()
        write_obj(filter_set)


main() if __name__ == "__main__" else None

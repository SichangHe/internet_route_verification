import sys

from rpsl_lexer import red
from rpsl_lexer.lex import mp_filter
from rpsl_lexer.lines import expressions
from rpsl_lexer.parse import clean_mp_filter, lex_with
from rpsl_lexer.piped import stdin_lines, write_obj
from rpsl_lexer.rpsl_object import FilterSet


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

import sys

from .dump import red
from .lex import mp_peering
from .lines import expressions
from .parse import clean_mp_peering, lex_with
from .piped import stdin_lines, write_obj
from .rpsl_object import PeeringSet


def parse_peering_set():
    peerings = []
    for key, expr in expressions(stdin_lines()):
        if key == "peering" or key == "mp-peering":
            try:
                lexed = lex_with(mp_peering, expr)
                if parsed := clean_mp_peering(lexed):
                    peerings.append(parsed)
            except Exception as err:
                tag = red("[parse_peering_set]")
                print(
                    f"{tag} {err} ParseException parsing `{expr}`.",
                    file=sys.stderr,
                )
    return PeeringSet("", "", peerings).__dict__


def main():
    print("Launching peering_set lexer.", file=sys.stderr)

    while True:
        peering_set = parse_peering_set()
        write_obj(peering_set)


main() if __name__ == "__main__" else None

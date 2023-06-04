import json

from pyparsing import sys

from .dump import parse_mp_import
from .lines import expressions, lines_continued
from .rpsl_object import AutNum


def parse_aut_num():
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    lines = stdin_lines()
    name = next(lines)
    for key, expr in expressions(lines_continued(lines)):
        if key == "import" or key == "mp-import":
            parse_mp_import(expr, imports)
        elif key == "export" or key == "mp-export":
            parse_mp_import(expr, exports)
    return AutNum(name, "", imports, exports).__dict__


def stdin_lines():
    line = ""
    while True:
        line += sys.stdin.read(1)
        if line.endswith("\n"):
            if len(line) == 1:
                break
            yield line[:-1]
            line = ""


def main():
    print("Launching aut_num lexer.", file=sys.stderr)

    while True:
        aut_num = parse_aut_num()
        json.dump(aut_num, sys.stdout)
        print()
        sys.stdout.flush()


main() if __name__ == "__main__" else None

import sys

from .dump import parse_mp_import
from .lines import expressions
from .piped import stdin_lines, write_obj
from .rpsl_object import AutNum


def parse_aut_num():
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    for key, expr in expressions(stdin_lines()):
        if key == "import":
            parse_mp_import(expr, imports)
        elif key == "mp-import":
            parse_mp_import(expr, imports, is_mp=True)
        elif key in ("export", "default"):
            parse_mp_import(expr, exports)
        elif key in ("mp-export", "mp-default"):
            parse_mp_import(expr, exports, is_mp=True)
    return AutNum("", "", imports, exports).__dict__


def main():
    print("Launching aut_num lexer.", file=sys.stderr)

    while True:
        aut_num = parse_aut_num()
        write_obj(aut_num)


main() if __name__ == "__main__" else None

import sys

from .lex import mp_import
from .lines import expressions
from .parse import import_export, lex_with
from .piped import stdin_lines, write_obj
from .rpsl_object import AutNum


def parse_mp_import(
    expr: str, imports: dict[str, dict[str, list[dict]]], is_mp: bool = False
):
    try:
        lexed = lex_with(mp_import, expr)
        import_export(lexed, imports, is_mp)
    except Exception as err:
        print(f"{err} parsing `{expr}`.")


def parse_aut_num():
    n_import, n_export = 0, 0
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    for key, expr in expressions(stdin_lines()):
        if key == "import":
            parse_mp_import(expr, imports)
            n_import += 1
        elif key == "mp-import":
            parse_mp_import(expr, imports, is_mp=True)
            n_import += 1
        elif key == "export":
            parse_mp_import(expr, exports)
            n_export += 1
        elif key == "mp-export":
            parse_mp_import(expr, exports, is_mp=True)
            n_export += 1
    return AutNum("", "", n_import, n_export, imports, exports).__dict__


def main():
    print("Launching aut_num lexer.", file=sys.stderr)

    while True:
        aut_num = parse_aut_num()
        write_obj(aut_num)


main() if __name__ == "__main__" else None

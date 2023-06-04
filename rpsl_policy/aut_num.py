import json

from pyparsing import sys

from .dump import parse_mp_import
from .lines import expressions, lines_continued
from .rpsl_object import AutNum, RPSLObject


def parse_aut_num(obj: RPSLObject):
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "import" or key == "mp-import":
            parse_mp_import(expr, imports)
        elif key == "export" or key == "mp-export":
            parse_mp_import(expr, exports)
    return AutNum(obj.name, obj.body, imports, exports).__dict__


def stdin_lines():
    line = ""
    while True:
        line += sys.stdin.read(1)
        if line.endswith("\n"):
            yield line
            line = ""


def main():
    print("Launching aut_num lexer.", file=sys.stderr)

    for line in stdin_lines():
        line = line.strip()
        raw = json.loads(line)
        obj = RPSLObject(raw["class"], raw["name"], raw["body"])
        json.dump(parse_aut_num(obj), sys.stdout)
        print()
        sys.stdout.flush()


main() if __name__ == "__main__" else None

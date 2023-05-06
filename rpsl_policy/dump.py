import json
import sys
from io import TextIOWrapper

from .lex import mp_import
from .lines import expressions, io_wrapper_lines, lines_continued, rpsl_objects
from .parse import import_export, lex_with
from .rpsl_object import AsSet, AutNum, RouteSet, RPSLObject

aut_nums: list[dict] = []
as_sets: list[dict] = []
route_sets: list[dict] = []

n = 0


def parse_mp_import(expr: str, imports: dict[str, dict[str, list[dict]]]):
    if lexed := lex_with(mp_import, expr):
        import_export(lexed, imports)


def parse_aut_num(obj: RPSLObject):
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "import" or key == "mp-import":
            parse_mp_import(expr, imports)
        elif key == "export" or key == "mp-export":
            parse_mp_import(expr, exports)
    aut_nums.append(AutNum(obj.name, obj.body, imports, exports).__dict__)


def gather_members(obj: RPSLObject) -> list[str]:
    members = []
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "members" or key == "mp-members":
            members.append(expr)
    return members


def print_count():
    print(
        f"Parsed {len(aut_nums)} aut_nums, {len(as_sets)} as_sets, {len(route_sets)} route_sets.",
        file=sys.stderr,
    )


def parse_object(obj: RPSLObject):
    global n
    if n % 0x100 == 0:
        print_count()
    if obj.closs == "aut-num":
        parse_aut_num(obj)
    if obj.closs == "as-set":
        members = gather_members(obj)
        as_sets.append(AsSet(obj.name, obj.body, members).__dict__)
    if obj.closs == "route-set":
        members = gather_members(obj)
        route_sets.append(RouteSet(obj.name, obj.body, members).__dict__)
    n += 1


def read_db(db: TextIOWrapper):
    db_lines = io_wrapper_lines(db)
    for obj in rpsl_objects(db_lines):
        parse_object(obj)
    json.dump(
        {"aut_nums": aut_nums, "as_sets": as_sets, "route_sets": route_sets},
        sys.stdout,
        separators=(",", ":"),
    )
    print_count()


def main():
    """Read and lex file whose name is read from command line arguments,
    and dump to Stdout."""
    filename = sys.argv[1]
    with open(filename, "r", encoding="latin-1") as db:
        read_db(db)


main() if __name__ == "__main__" else None

import sys
from io import TextIOWrapper

from pyparsing import ParseException

from .lex import mp_import
from .lines import expressions, io_wrapper_lines, lines_continued, rpsl_objects
from .parse import import_export
from .rpsl_object import AsSet, AutNum, RouteSet, RPSLObject

aut_nums: list[AutNum] = []
as_sets: list[AsSet] = []
route_sets: list[RouteSet] = []


def parse_mp_import(expr: str):
    try:
        lexed = mp_import.parse_string(expr).as_dict()
    except ParseException as err:
        print(err, file=sys.stderr)
        return
    return import_export(lexed)


def parse_aut_num(obj: RPSLObject):
    imports = []
    exports = []
    for key, expr in expressions(obj.body.splitlines()):
        if key == "import" or key == "mp-import":
            imports.append(parse_mp_import(expr))
        elif key == "export" or key == "mp-export":
            exports.append(parse_mp_import(expr))
    aut_nums.append(AutNum(obj.name, obj.body, imports, exports))


def gather_members(obj: RPSLObject) -> list[str]:
    members = []
    for key, expr in expressions(obj.body.splitlines()):
        if key == "members" or key == "mp-members":
            members.append(expr)
    return members


def parse_object(obj: RPSLObject):
    if obj.closs == "aut-num":
        parse_aut_num(obj)
    if obj.closs == "as-set":
        members = gather_members(obj)
        as_sets.append(AsSet(obj.name, obj.body, members))
    if obj.closs == "route-set":
        members = gather_members(obj)
        route_sets.append(RouteSet(obj.name, obj.body, members))


def read_db(db: TextIOWrapper):
    db_lines = io_wrapper_lines(db)
    for obj in rpsl_objects(lines_continued(db_lines)):
        parse_object(obj)


def main():
    """Read and lex file whose name is read from command line arguments,
    and dump to Stdout."""
    filename = sys.argv[1]
    with open(filename, "r", encoding="latin-1") as db:
        read_db(db)


main() if __name__ == "__main__" else None

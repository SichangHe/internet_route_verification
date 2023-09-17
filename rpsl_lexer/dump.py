import json
import sys
from io import TextIOWrapper

from pyparsing import ParseException

from .lex import member, mp_import, mp_peering
from .lines import expressions, io_wrapper_lines, lines_continued, rpsl_objects
from .parse import clean_mp_peering, import_export, lex_with
from .rpsl_object import AsSet, AutNum, PeeringSet, RouteSet, RPSLObject

aut_nums: list[dict] = []
as_sets: list[dict] = []
route_sets: list[dict] = []
peering_sets: list[dict] = []

n = 0


def red(string: str) -> str:
    return f"\033[91m{string}\033[0m"


def parse_mp_import(
    expr: str, imports: dict[str, dict[str, list[dict]]], is_mp: bool = False
):
    try:
        lexed = lex_with(mp_import, expr)
        import_export(lexed, imports, is_mp)
    except Exception as err:
        print(f"{err} parsing `{expr}`.")


def parse_aut_num(obj: RPSLObject):
    imports: dict[str, dict[str, list[dict]]] = {}
    exports: dict[str, dict[str, list[dict]]] = {}
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "import" or key == "mp-import":
            parse_mp_import(expr, imports)
        elif key in ("export", "mp-export", "default", "mp-default"):
            parse_mp_import(expr, exports)
    aut_nums.append(AutNum(obj.name, obj.body, imports, exports).__dict__)


def gather_members(obj: RPSLObject) -> list[str]:
    members = []
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "members" or key == "mp-members":
            try:
                lexed = member.parse_string(expr, parse_all=True)
            except ParseException:
                tag = red("[gather_members]")
                print(
                    f"{tag} ParseException parsing `{expr}` in {obj}.",
                    file=sys.stderr,
                )
                continue
            members.extend(lexed.as_list())
    return members


def gather_peerings(obj: RPSLObject) -> list[dict]:
    peerings = []
    for key, expr in expressions(lines_continued(obj.body.splitlines())):
        if key == "peering" or key == "mp-peering":
            try:
                lexed = lex_with(mp_peering, expr)
                if parsed := clean_mp_peering(lexed):
                    peerings.append(parsed)
            except Exception:
                tag = red("[gather_peerings]")
                print(
                    f"{tag} ParseException parsing `{expr}` in {obj}.",
                    file=sys.stderr,
                )
    return peerings


def print_count():
    print(
        f"Parsed {len(aut_nums)} aut_nums, {len(as_sets)} as_sets, {len(route_sets)} route_sets, {len(peering_sets)} peering_sets.",
        file=sys.stderr,
    )


def parse_object(obj: RPSLObject):
    global n
    if n % 0x1000 == 0:
        print_count()
    if obj.closs == "aut-num":
        parse_aut_num(obj)
    if obj.closs == "as-set":
        members = gather_members(obj)
        as_sets.append(AsSet(obj.name, obj.body, members).__dict__)
    if obj.closs == "route-set":
        members = gather_members(obj)
        route_sets.append(RouteSet(obj.name, obj.body, members).__dict__)
    if obj.closs == "peering-set":
        peerings = gather_peerings(obj)
        peering_sets.append(PeeringSet(obj.name, obj.body, peerings).__dict__)
    n += 1


def read_db(db: TextIOWrapper):
    db_lines = io_wrapper_lines(db)
    for obj in rpsl_objects(db_lines):
        parse_object(obj)
    json.dump(
        {
            "aut_nums": aut_nums,
            "as_sets": as_sets,
            "route_sets": route_sets,
            "peering_sets": peering_sets,
        },
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

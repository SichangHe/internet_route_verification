import json
import sys


def stdin_lines():
    line = ""
    while True:
        line += sys.stdin.read(1)
        if line.endswith("\n"):
            if len(line) == 1:
                break
            yield line[:-1]
            line = ""


def write_obj(obj):
    json.dump(obj, sys.stdout)
    print()
    sys.stdout.flush()

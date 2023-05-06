import re
from io import TextIOWrapper
from typing import Generator, Iterable

from .rpsl_object import RPSLObject

continuation_chars = (" ", "+", "\t")
spaces = re.compile(r"\s+")


def io_wrapper_lines(reader: TextIOWrapper) -> Generator[str, None, None]:
    """Lazily read `reader` line by line."""
    while line := reader.readline():
        yield line


def lines_continued(raw_lines: Iterable[str]) -> Generator[str, None, None]:
    """Merge continued RPSL lines in `raw_lines` into single lines according to
    prefix continuation characters and yield them one by one.
    Strip and ignore comments. Ignore empty lines."""
    last_line: str = ""
    line: str = ""
    for line in raw_lines:
        # Remove comments.
        line = line.split("#", maxsplit=1)[0]
        # Handle continuation lines.
        if line.startswith(continuation_chars):
            last_line += " " + line[1:].strip()
            continue
        # Not a continuation line.
        if last_line:
            yield last_line
        last_line = line.strip()
    if last_line:
        yield last_line


def cleanup_right_whitespace(string: str):
    string = string.rstrip()
    return re.sub(spaces, " ", string)


def cleanup_whitespace(string: str):
    string = string.strip()
    return re.sub(spaces, " ", string)


def rpsl_objects(lines: Iterable[str]) -> Generator[RPSLObject, None, None]:
    """Combine lines from an iterator into RPSL objects."""
    closs = ""
    """Class"""
    name = ""
    body = ""
    new = False
    for line in lines:
        line = cleanup_right_whitespace(line)
        if line == "":
            # Empty line suggests the end of the last object.
            if new:
                continue
            # Yield the last object.
            yield RPSLObject(closs, name, body)
            new = True
            continue
        if not new:
            body += line + "\n"
            continue
        # Start of new object.
        if ":" not in line:
            raise RuntimeError(f"Invalid line: `{line}`.")
        closs, name = map(cleanup_whitespace, line.split(":", maxsplit=1))
        new = False
        body = ""
    if not new:
        # The last line is not empty.
        yield RPSLObject(closs, name, body)


def expressions(lines: Iterable[str]) -> Generator[tuple[str, str], None, None]:
    for line in lines:
        if ":" not in line:
            raise RuntimeError(f"Invalid line: `{line}`.")
        key, expression = map(str.strip, line.split(":", maxsplit=1))
        yield key, expression

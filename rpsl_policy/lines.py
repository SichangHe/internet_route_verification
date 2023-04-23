from typing import Generator, Iterable

continuation_chars = (" ", "+", "\t")


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

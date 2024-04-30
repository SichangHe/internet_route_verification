"""Run at `scripts/` with `python3 -m scripts.stats.route_objects`."""

import gzip
import json
from typing import TypedDict

from scripts.csv_files import route_objects_defined_multiple_times

N_PREFIX = 2817344
"""From <https://github.com/SichangHe/internet_route_verification/issues/138#issuecomment-2016682503>."""


class Route(TypedDict):
    name: str
    origin: str
    mntby: str
    source: str


def main() -> None:
    route_objects_defined_multiple_times.download_if_missing()
    with gzip.open(route_objects_defined_multiple_times.path, "rt") as file:
        prefixes_defined_multiple_times: dict[str, list[Route]] = json.load(file)

    n_prefix_defined_multiple_times = len(prefixes_defined_multiple_times)
    n_prefix_defined_once = N_PREFIX - n_prefix_defined_multiple_times
    n_route_of_prefix_defined_multiple_times = sum(
        len(routes) for routes in prefixes_defined_multiple_times.values()
    )
    n_route = n_route_of_prefix_defined_multiple_times + n_prefix_defined_once
    print(
        f"""{n_route:,} ({n_prefix_defined_once:,} + {n_route_of_prefix_defined_multiple_times:,}) route objects for \
{n_prefix_defined_once:,} prefixes defined only once and \
{n_prefix_defined_multiple_times:,} prefixes defined multiple times."""
    )


main() if __name__ == "__main__" else None

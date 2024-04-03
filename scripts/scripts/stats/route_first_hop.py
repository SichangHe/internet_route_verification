"""Run at `scripts/` with `python3 -m scripts.stats.route_first_hop`.
Stats for the first hop in each AS-path.
This takes more RAM than ordinary laptops have.
"""

from concurrent import futures

import pandas as pd

from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_files import route_first_hop_stats_all as FILES

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def read_route_stats(file: CsvFile):
    return pd.read_csv(file.path, dtype="uint16", engine="pyarrow")


def read_all_route_stats() -> pd.DataFrame:
    with futures.ProcessPoolExecutor() as executor:
        return pd.concat(
            (d for d in executor.map(read_route_stats, FILES) if len(d) > 0),
            copy=False,
        )


def main() -> None:
    download_csv_files_if_missing(FILES)

    df = read_all_route_stats()

    print(df.describe().to_string())

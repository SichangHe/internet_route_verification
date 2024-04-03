"""Run at `scripts/` with `python3 -m scripts.stats.route_first_hop`.
Stats for the first hop in each AS-path.
This takes more RAM than ordinary laptops have.
"""

from concurrent import futures

from dask import dataframe as dd

from scripts import CsvFile
from scripts.csv_files import route_first_hop_stats_all as FILES

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def read_route_stats(file: CsvFile):
    return dd.read_csv(file.path, dtype="uint16", blocksize=None, engine="pyarrow")


def read_all_route_stats():
    return dd.concat(
        (d for d in [read_route_stats(file) for file in FILES] if len(d) > 0),
    )


def main() -> None:
    with futures.ThreadPoolExecutor() as executor:
        executor.map(CsvFile.download_if_missing, FILES)

    df = dd.concat([read_route_stats(file) for file in FILES])

    print(df.describe().compute().to_string())


main() if __name__ == "__main__ " else None

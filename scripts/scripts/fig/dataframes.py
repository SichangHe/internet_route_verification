from concurrent import futures
from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_files import as_stats_all
import pandas as pd
from itertools import repeat


def as_stats_all_df(usecols: list[str]):
    download_csv_files_if_missing(as_stats_all)
    with futures.ProcessPoolExecutor() as executor:
        dfs = executor.map(CsvFile.read_w_default_config, as_stats_all, repeat(usecols))
        concatenated = pd.concat((d for d in dfs if len(d) > 0), copy=False)
        return concatenated.groupby("aut_num").sum(engine="pyarrow")  # type: ignore

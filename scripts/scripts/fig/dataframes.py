from concurrent import futures
from itertools import repeat

import pandas as pd

from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_files import as_pair_stats_all, as_stats_all


def as_stats_all_df(usecols: list[str]):
    download_csv_files_if_missing(as_stats_all)
    with futures.ProcessPoolExecutor() as executor:
        dfs = executor.map(CsvFile.read_w_default_config, as_stats_all, repeat(usecols))
        concatenated = pd.concat((d for d in dfs if len(d) > 0), copy=False)
    df = concatenated.groupby("aut_num").sum(engine="pyarrow")  # type: ignore[reportArgumentType]
    assert isinstance(df, pd.DataFrame)
    return df


def as_pair_stats_all_df(usecols: list[str]):
    download_csv_files_if_missing(as_pair_stats_all)
    with futures.ProcessPoolExecutor() as executor:
        dfs = executor.map(
            CsvFile.read_w_default_config, as_pair_stats_all, repeat(usecols)
        )
        concatenated = pd.concat((d for d in dfs if len(d) > 0), copy=False)
    df = concatenated.groupby(["from", "to"]).sum(engine="pyarrow")  # type: ignore[reportArgumentType]
    assert isinstance(df, pd.DataFrame)
    return df

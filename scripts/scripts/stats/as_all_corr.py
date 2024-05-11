"""Run at `scripts/` with `python3 -m scripts.stats.as_all_corr`."""

from concurrent import futures

import pandas as pd

from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_files import as_neighbors_vs_rules, as_stats_all

NEIGHBORS = ("provider", "peer", "customer")
RULES = ("import", "export")
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def read_as_stats(file: CsvFile):
    return pd.read_csv(
        file.path,
        dtype="uint",
        engine="pyarrow",
    )


def main():
    as_neighbors_vs_rules.download_if_missing()
    anr_raw = pd.read_csv(as_neighbors_vs_rules.path)
    # Remove ASes not in IRR or not in AS Relationship DB.
    anr = anr_raw.drop(
        anr_raw[(anr_raw["import"] == -1) | (anr_raw["provider"] == -1)].index
    )

    download_csv_files_if_missing(as_stats_all)
    with futures.ProcessPoolExecutor() as executor:
        asst = (
            pd.concat(executor.map(read_as_stats, as_stats_all), copy=False)
            .groupby("aut_num")
            .sum(engine="pyarrow")
        )

    df = asst.merge(anr, on="aut_num")

    df["neighbor"] = sum(df[neighbor] for neighbor in NEIGHBORS)
    df["rules"] = sum(df[rule] for rule in RULES)
    df["rule_by_neighbor"] = df["rules"] / df["neighbor"]

    for port in PORTS:
        df[f"{port}_total"] = sum(df[f"{port}_{tag}"] for tag in TAGS)
        for tag in TAGS:
            df[f"%{port}_{tag}"] = (
                df[f"{port}_{tag}"] / df[f"{port}_total"].replace(0.0, 1.0) * 100.0
            )
    df["exchange_total"] = sum(df[f"{port}_{tag}"] for tag in TAGS for port in PORTS)
    for tag in TAGS:
        df[f"%{tag}"] = (
            sum(df[f"{port}_{tag}"] for port in PORTS)
            / df["exchange_total"].replace(0.0, 1.0)
            * 100.0
        )

    columns = df.columns
    correlation_pairs = []
    for i, col1 in enumerate(columns):
        for j, col2 in enumerate(columns):
            if i > j:
                correlation = df[col1].corr(df[col2])
                correlation_pairs.append((col1, col2, correlation))
    corr_df = pd.DataFrame(
        correlation_pairs, columns=["Column1", "Column2", "Correlation"]
    )
    corr_df.dropna(inplace=True)  # Rid invariances.

    print("All Pearson correlations coefficients:")
    print(corr_df.to_string())

    print("Significant Pearson correlations coefficients:")
    print(corr_df[abs(corr_df["Correlation"]) > 0.7].to_string())


if __name__ == "__main__":
    main()

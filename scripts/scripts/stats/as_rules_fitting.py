"""Run at `scripts/` with `python3 -m scripts.stats.as_rules_fitting`.

WIP
"""

import pandas as pd
from fitter import Fitter

from scripts.csv_files import as_neighbors_vs_rules

FILE = as_neighbors_vs_rules
NEIGHBORS = ("provider", "peer", "customer")
RULES = ("import", "export")


def main():
    FILE.download_if_missing()
    df_raw = pd.read_csv(FILE.path)
    # Remove ASes not in IRR or not in AS Relationship DB.
    df = df_raw.drop(
        df_raw[(df_raw["import"] == -1) | (df_raw["provider"] == -1)].index
    )
    df["neighbors"] = sum(df[neighbor] for neighbor in NEIGHBORS)
    df["rules"] = sum(df[rule] for rule in RULES)

    fitter = Fitter(df["rules"], bins=10000, timeout=3600)
    fitter.fit(progress=True)
    summary = fitter.summary(Nbest=111, plot=False)
    print(summary.to_string())
    print(fitter.fitted_param)


if __name__ == "__main__":
    main()

"""Run at `scripts/` with `python3 -m scripts.stats.as_rules_corr`."""

import pandas as pd

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

    neighbors = list(NEIGHBORS) + ["neighbors"]
    rules = list(RULES) + ["rules"]
    print("Pearson, Kendall Tau, and Spearman rank correlation.")
    for neighbor in neighbors:
        for rule in rules:
            corrs = ", ".join(
                [
                    f"{df[neighbor].corr(df[rule], method=method):.3f}"
                    for method in ("pearson", "kendall", "spearman")
                ]
            )
            print(f"{corrs}: between {neighbor} and {rule}.")

    df = df[(df["customer"] >= 5) & (df["rules"] > 0)]
    print(
        "Filtering only transit ASes with at least 5 customers and 1 rule, Pearson, Kendall Tau, and Spearman rank correlation."
    )
    for neighbor in neighbors:
        for rule in rules:
            corrs = ", ".join(
                [
                    f"{df[neighbor].corr(df[rule], method=method):.3f}"
                    for method in ("pearson", "kendall", "spearman")
                ]
            )
            print(f"{corrs}: between {neighbor} and {rule}.")


if __name__ == "__main__":
    main()

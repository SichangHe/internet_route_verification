"""Run at `scripts/` with `python3 -m scripts.stats.as_pair_all_some`.

Adopted from `as_all_some`.
"""

import pandas as pd

from scripts.csv_files import as_pair_stats

FILE = as_pair_stats
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def main():
    FILE.download_if_missing()
    df = pd.read_csv(FILE.path)
    n_asp = len(df)
    print(f"{n_asp} AS pairs in total.")

    df["total"] = sum((df[f"{port}_{tag}"] for tag in TAGS for port in PORTS))
    df_all = {}
    df_some = {}
    for tag in TAGS:
        df_all[tag] = df[
            df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]
        ].dropna()
        count = df_all[tag].__len__()
        percentage = count / n_asp * 100
        print(f"{count} all {tag}, {percentage:.2f}%.")

    print()
    for tag in TAGS:
        df_some[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0].dropna()
        count = df_some[tag].__len__()
        percentage = count / n_asp * 100
        print(f"{count} have {tag}, {percentage:.2f}%.")

    for port in PORTS:
        print()
        df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))

        df_all[f"{port}_dne"] = df[df[f"total_{port}"] == 0].dropna()
        n_dne = df_all[f"{port}_dne"].__len__()
        percentage = n_dne / n_asp * 100
        n_e = n_asp - n_dne
        print(f"{n_dne} have no {port}, {percentage:.2f}%; {n_e} have {port}.")

        for tag in TAGS:
            df_all[f"{port}_{tag}"] = df[
                (df[f"total_{port}"] != 0)
                & (df[f"{port}_{tag}"] == df[f"total_{port}"])
            ].dropna()
            count = df_all[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} all {tag} in {port}, {percentage:.2f}% among AS pairs with {port}."
            )

        print()
        for tag in TAGS:
            df_some[f"{port}_{tag}"] = df[df[f"{port}_{tag}"] > 0].dropna()
            count = df_some[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} have {tag} in {port}, {percentage:.2f}% among AS pairs with {port}."
            )


if __name__ == "__main__":
    main()

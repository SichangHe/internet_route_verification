"""Run at `scripts/` with `python3 -m scripts.stats.as_pair_all_all_some`.

Adopted from `as_pair_spec_all_all_some`.
"""

from scripts.fig.dataframes import as_pair_stats_all_df

PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    df = as_pair_stats_all_df(
        ["from", "to"] + [f"{port}_{tag}" for tag in LEVELS for port in PORTS]
    )
    n_as_pair = len(df)
    print(f"{n_as_pair} AS pairs in total.\n")

    df["total"] = sum((df[f"{port}_{tag}"] for tag in LEVELS for port in PORTS))

    df_all = {}
    count_all = 0
    for tag in LEVELS:
        df_all[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]]
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as_pair
        print(f"{count} all {tag}, {percentage:.2f}%.")
        count_all += count
    percentage = count_all * 100 / n_as_pair
    print(f"{count_all} all same status, {percentage:.2f}%.\n")

    df_some = {}
    for tag in LEVELS:
        df_some[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0].dropna()
        count = df_some[tag].__len__()
        percentage = count * 100 / n_as_pair
        print(f"{count} have {tag}, {percentage:.2f}%.")

    for port in PORTS:
        print()
        df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in LEVELS))
        count_all = 0

        df_all[f"{port}_dne"] = df[df[f"total_{port}"] == 0].dropna()
        n_dne = df_all[f"{port}_dne"].__len__()
        percentage = n_dne / n_as_pair * 100
        n_e = n_as_pair - n_dne
        print(f"{n_dne} have no {port}, {percentage:.2f}%; {n_e} have {port}.")

        for tag in LEVELS:
            df_all[f"{port}_{tag}"] = df[
                (df[f"total_{port}"] != 0)
                & (df[f"{port}_{tag}"] == df[f"total_{port}"])
            ].dropna()
            count = df_all[f"{port}_{tag}"].__len__()
            percentage = count * 100 / n_e
            print(
                f"{count} all {tag} in {port}, {percentage:.2f}% among AS pairs with {port}."
            )
            count_all += count
        percentage = count_all * 100 / n_e
        print(
            f"{count_all} all same status in {port}, {percentage:.2f}% among AS pairs with {port}.\n"
        )

        for tag in LEVELS:
            df_some[f"{port}_{tag}"] = df[df[f"{port}_{tag}"] > 0].dropna()
            count = df_some[f"{port}_{tag}"].__len__()
            percentage = count * 100 / n_e
            print(
                f"{count} have {tag} in {port}, {
                    percentage:.2f}% among AS pairs with {port}."
            )


if __name__ == "__main__":
    main()

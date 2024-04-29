"""Run at `scripts/` with `python3 -m scripts.stats.as_all_all_some`.

Adopted from `as_all_some`.
"""

from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    df = as_stats_all_df(
        [f"{port}_{tag}" for tag in TAGS for port in PORTS] + ["aut_num"]
    )
    n_as = len(df)
    print(f"{n_as} ASes in total.")

    df["total"] = sum((df[f"{port}_{tag}"] for tag in TAGS for port in PORTS))
    df_all = {}
    count_all = 0
    for tag in TAGS:
        df_all[tag] = df[
            df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]
        ].dropna()
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} all {tag}, {percentage:.1f}%.")
        count_all += count
    percentage = count_all * 100 / n_as
    print(f"{count_all} all same status, {percentage:.1f}%.\n")

    df_some = {}
    for tag in TAGS:
        df_some[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0].dropna()
        count = df_some[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} have {tag}, {percentage:.1f}%.")

    for port in PORTS:
        print()
        df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))

        df_all[f"{port}_dne"] = df[df[f"total_{port}"] == 0].dropna()
        n_dne = df_all[f"{port}_dne"].__len__()
        percentage = n_dne / n_as * 100
        n_e = n_as - n_dne
        print(f"{n_dne} have no {port}, {percentage:.1f}%; {n_e} have {port}.")

        for tag in TAGS:
            df_all[f"{port}_{tag}"] = df[
                (df[f"total_{port}"] != 0)
                & (df[f"{port}_{tag}"] == df[f"total_{port}"])
            ].dropna()
            count = df_all[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} all {tag} in {port}, {percentage:.1f}% among ASes with {port}."
            )

        print()
        for tag in TAGS:
            df_some[f"{port}_{tag}"] = df[df[f"{port}_{tag}"] > 0].dropna()
            count = df_some[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} have {tag} in {port}, {percentage:.1f}% among ASes with {port}."
            )


if __name__ == "__main__":
    main()

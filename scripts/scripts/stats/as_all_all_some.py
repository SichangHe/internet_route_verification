"""Run at `scripts/` with `python3 -m scripts.stats.as_all_all_some`.

Adopted from `as_all_some`.
"""

from scripts.csv_fields import SAFELIST_REPORT_ITEM_FIELDS
from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")
NON_SKIP_TAGS = ("ok", "meh", "err")


def main() -> None:
    df = as_stats_all_df(
        [f"{port}_{tag}" for tag in TAGS for port in PORTS]
        + ["aut_num"]
        + list(SAFELIST_REPORT_ITEM_FIELDS)
    )
    df["safelisted"] = sum(df[tag] for tag in SAFELIST_REPORT_ITEM_FIELDS)
    df["relaxed"] = sum(df[f"{port}_meh"] for port in PORTS) - df["safelisted"]
    n_as = len(df)
    print(f"{n_as} ASes in total.")

    df["total"] = sum((df[f"{port}_{tag}"] for tag in TAGS for port in PORTS))
    df["total_non_skips"] = sum(
        (df[f"{port}_{tag}"] for tag in NON_SKIP_TAGS for port in PORTS)
    )
    df_non_skip = df[df["total_non_skips"] == df["total"]]
    n_non_skip_as = len(df_non_skip)
    print(f"{n_non_skip_as} ASes without skip or unrec.\n")

    df_all = {}
    count_all = 0
    count_all_non_skip = 0
    for tag in ("ok", "skip", "unrec", "err"):
        df_all[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]]
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as
        percentage_non_skip = 0
        if tag in NON_SKIP_TAGS:
            percentage_non_skip = count * 100 / n_non_skip_as
            count_all_non_skip += count
        print(
            f"{count} all {tag}, {percentage:.2f}% among all, {
                percentage_non_skip:.2f}% excluding skips."
        )
        count_all += count

    for tag in ("relaxed", "safelisted"):
        df_all[tag] = df[df[tag] == df["total"]]
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as
        percentage_non_skip = count * 100 / n_non_skip_as
        print(
            f"{count} all {tag}, {percentage:.2f}% among all, {
                percentage_non_skip:.2f}% excluding skips."
        )
        count_all += count
    percentage = count_all * 100 / n_as
    percentage_non_skip = count_all_non_skip * 100 / n_non_skip_as
    print(
        f"{count_all} all same status, {percentage:.2f}% among all, {
            percentage_non_skip:.2f}% excluding skips.\n"
    )

    df_some = {}
    df_some_non_skip = {}
    for tag in TAGS:
        df_some[tag] = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0].dropna()
        count = df_some[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} have {tag}, {percentage:.2f}%.")
        if tag in NON_SKIP_TAGS:
            df_some_non_skip[tag] = df_non_skip[
                df_non_skip[f"import_{tag}"] + df_non_skip[f"export_{tag}"] > 0
            ]
            count = df_some_non_skip[tag].__len__()
            percentage_non_skip = count * 100 / n_non_skip_as
            print(
                f"{count} excluding ASes with skips have {
                    tag}, {percentage_non_skip:.2f}%."
            )

    for tag in ("relaxed", "safelisted"):
        df_some[tag] = df[df[tag] > 0]
        count = df_some[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} have {tag}, {percentage:.2f}%.")

        df_some_non_skip[tag] = df_non_skip[df_non_skip[tag] > 0]
        count = df_some_non_skip[tag].__len__()
        percentage_non_skip = count * 100 / n_non_skip_as
        print(
            f"{count} excluding ASes with skips have {
                tag}, {percentage_non_skip:.2f}%."
        )

    for port in PORTS:
        print()
        df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))

        df_all[f"{port}_dne"] = df[df[f"total_{port}"] == 0].dropna()
        n_dne = df_all[f"{port}_dne"].__len__()
        percentage = n_dne / n_as * 100
        n_e = n_as - n_dne
        print(f"{n_dne} have no {port}, {percentage:.2f}%; {n_e} have {port}.")

        for tag in TAGS:
            df_all[f"{port}_{tag}"] = df[
                (df[f"total_{port}"] != 0)
                & (df[f"{port}_{tag}"] == df[f"total_{port}"])
            ].dropna()
            count = df_all[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} all {tag} in {port}, {
                    percentage:.2f}% among ASes with {port}."
            )

        print()
        for tag in TAGS:
            df_some[f"{port}_{tag}"] = df[df[f"{port}_{tag}"] > 0].dropna()
            count = df_some[f"{port}_{tag}"].__len__()
            percentage = count / n_e * 100
            print(
                f"{count} have {tag} in {port}, {
                    percentage:.2f}% among ASes with {port}."
            )


if __name__ == "__main__":
    main()

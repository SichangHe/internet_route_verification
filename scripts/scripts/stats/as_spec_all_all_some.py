"""Run at `scripts/` with `python3 -m scripts.stats.as_spec_all_all_some`.

Adopted from `as_unrec_all_breakdown` and `as_all_all_some`.
"""

from scripts.csv_fields import SPECIAL_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    df = as_stats_all_df(
        [f"{port}_{tag}" for tag in LEVELS for port in PORTS] + list(TAGS) + ["aut_num"]
    )
    n_as = len(df)
    print(f"{n_as} ASes in total.")

    df["total_report"] = sum((df[f"{port}_{tag}"] for tag in LEVELS for port in PORTS))
    df["total_meh"] = df["import_meh"] + df["export_meh"]
    df["meh_rate"] = df["total_meh"] / df["total_report"]

    raw_df = df  # For interactive use.
    _ = raw_df
    df = df[df["total_meh"] > 0]
    n_meh = len(df)
    percentage = n_meh * 100 / n_as
    print(f"{n_meh} ASes with specal or whitelisted cases, {percentage:.1f}%.")

    df_all = {}
    count_all = 0
    for tag in TAGS:
        df_all[tag] = df[df[tag] == df["total_report"]]
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} all {tag}, {percentage:.1f}%.")
        count_all += count
    percentage = count_all * 100 / n_as
    print(
        f"{count_all} all same special/whitelisted case subtype, {percentage:.1f}%.\n"
    )

    df_spec_all = {}
    count_all = 0
    for tag in TAGS:
        df_spec_all[tag] = df[df[tag] == df["total_meh"]]
        count = df_all[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} all {tag} among special/whitelisted cases, {percentage:.1f}%.")
        count_all += count
    percentage = count_all * 100 / n_as
    print(
        f"{count_all} all same subtype among special/whitelisted case, {percentage:.1f}%.\n"
    )

    df_some = {}
    for tag in TAGS:
        df_some[tag] = df[df[tag] > 0]
        count = df_some[tag].__len__()
        percentage = count * 100 / n_as
        print(f"{count} have {tag}, {percentage:.1f}%.")


if __name__ == "__main__":
    main()

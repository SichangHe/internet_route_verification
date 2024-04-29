"""Run at `scripts/` with `python3 -m scripts.stats.as_unrec_all_breakdown`.

Adopted from `as_all_all_some`.
"""

from scripts.csv_fields import UNRECORDED_CASE_REPORT_ITEM_FIELDS as TAGS
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
    df["total_unrec"] = df["import_unrec"] + df["export_unrec"]
    df["unrec_rate"] = df["total_unrec"] / df["total_report"]
    df["unrec_route"] = sum(
        (
            df[tag]
            for tag in (
                "unrec_as_set_route",
                "unrec_some_as_set_route",
                "unrec_as_routes",
            )
        )
    )
    raw_df = df  # For interactive use.
    _ = raw_df
    df = df[df["unrec_rate"] > 0]
    n_unrec = len(df)
    percentage = n_unrec * 100 / n_as
    print(f"{n_unrec} ASes with unrecorded cases, {percentage:.1f}%.")

    df_all = {}
    # %ASes missing an \autnum object
    df_all["missing_autnum"] = df[df["unrec_aut_num"] > 0]
    count = df_all["missing_autnum"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} missing aut-num, {percentage:.1f}%.")

    # %\autnum objects with 0 \import and \export rule
    df_all["zero_import"] = df[(df["unrec_import_empty"] > 0)]
    count = df_all["zero_import"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} with 0 import rule, {percentage:.1f}%.")

    df_all["zero_export"] = df[(df["unrec_export_empty"] > 0)]
    count = df_all["zero_export"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} with 0 export rule, {percentage:.1f}%.")

    df_all["zero_rule"] = df[
        (df["unrec_import_empty"] > 0) & (df["unrec_export_empty"]) > 0
    ]
    count = df_all["zero_rule"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} with 0 import and 0 export rule, {percentage:.1f}%.")

    df_all["either_zero_rule"] = df[
        (df["unrec_import_empty"] > 0) | (df["unrec_export_empty"]) > 0
    ]
    count = df_all["either_zero_rule"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} with either 0 import or 0 export rule, {percentage:.1f}%.")

    # %ASes referring to ASes without originating \route objects.
    df_all["no_route"] = df[(df["unrec_route"] > 0)]
    count = df_all["no_route"].__len__()
    percentage = count * 100 / n_as
    print(
        f"{count} refer to ASes without originating route in filter, {percentage:.1f}%."
    )

    # %ASes making active use of the RPSL have rules that refer to missing RPSL
    # objects.
    df_all["missing_object"] = df[
        df["total_unrec"]
        - (
            df["unrec_import_empty"]
            + df["unrec_export_empty"]
            + df["unrec_aut_num"]
            + df["unrec_route"]
        )
        > 0
    ]
    count = df_all["missing_object"].__len__()
    percentage = count * 100 / n_as
    print(f"{count} refer to missing RPSL objects, {percentage:.1f}%.")


if __name__ == "__main__":
    main()

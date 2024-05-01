"""Run at `scripts/` with `python3 -m scripts.stats.route_unrec_all_breakdown`.
Takes ~12min.

Adopted from `route_spec_all_all_some`.
"""

from typing import DefaultDict

from scripts import download_csv_files_if_missing
from scripts.csv_fields import UNRECORDED_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.csv_files import route_stats_all

PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    download_csv_files_if_missing(route_stats_all)
    usecols = [f"{port}_{tag}" for tag in LEVELS for port in PORTS] + list(TAGS)

    n_route = 0
    n_unrec = 0
    counts = DefaultDict(int)
    for csv_file in route_stats_all:
        df = csv_file.read_w_default_config(usecols)
        n_route += len(df)

        df["total"] = sum((df[f"{port}_{tag}"] for tag in LEVELS for port in PORTS))
        df["total_unrec"] = df["import_unrec"] + df["export_unrec"]
        df = df[df["total_unrec"] > 0]
        n_unrec += len(df)
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

        for tag in TAGS:
            d = df[df[tag] == df["total"]]
            count = len(d)
            counts[f"all_{tag}"] += count
            counts["all_same"] += count

            d = df[df[tag] == df["total_unrec"]]
            count = len(d)
            counts[f"unrec_all_{tag}"] += count
            counts[f"unrec_all_same"] += count

            d = df[df[tag] > 0]
            counts[f"some_{tag}"] += len(d)

        d = df[(df["unrec_import_empty"] > 0) & (df["unrec_export_empty"]) > 0]
        counts["zero_rule"] += len(d)

        d = df[(df["unrec_import_empty"] > 0) | (df["unrec_export_empty"]) > 0]
        counts["either_zero_rule"] += len(d)

        d = df[(df["unrec_route"] > 0)]
        counts["unrec_route"] += len(d)

        d = df[
            df["total_unrec"]
            - (
                df["unrec_import_empty"]
                + df["unrec_export_empty"]
                + df["unrec_aut_num"]
                + df["unrec_route"]
            )
            > 0
        ]
        counts["missing_object"] += len(d)

        del df

    print(f"{n_route} routes in total, {n_unrec} have unrecorded cases.")
    for tag in TAGS:
        count = counts[f"all_{tag}"]
        percentage = count * 100 / n_route
        print(f"{count} all {tag}, {percentage:.1f}%.")
    count = counts["all_same"]
    percentage = count * 100 / n_route
    print(f"{count} all same status, {percentage:.1f}%.\n")

    for tag in TAGS:
        count = counts[f"unrec_all_{tag}"]
        percentage = count * 100 / n_route
        print(f"{count} all {tag} among unrecorded cases, {percentage:.1f}%.")
    count = counts["unrec_all_same"]
    percentage = count * 100 / n_route
    print(f"{count} all same subtype among unrecorded cases, {percentage:.1f}%.\n")

    for tag in TAGS:
        count = counts[f"some_{tag}"]
        percentage = count * 100 / n_route
        print(f"{count} have {tag}, {percentage:.1f}%.")

    count = counts["zero_rule"]
    percentage = count * 100 / n_route
    print(f"{count} with 0 import and 0 export rule, {percentage:.1f}%.")

    count = counts["either_zero_rule"]
    percentage = count * 100 / n_route
    print(f"{count} with either 0 import or 0 export rule, {percentage:.1f}%.")

    count = counts["unrec_route"]
    percentage = count * 100 / n_route
    print(
        f"{count} refer to ASes without originating route in filter, {percentage:.1f}%."
    )

    count = counts["missing_object"]
    percentage = count * 100 / n_route
    print(f"{count} refer to missing RPSL objects, {percentage:.1f}%.")


if __name__ == "__main__":
    main()

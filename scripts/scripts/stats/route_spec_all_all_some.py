"""Run at `scripts/` with `python3 -m scripts.stats.route_spec_all_all_some`.
Takes ~12min.

Adopted from `as_all_all_some`.
"""

from typing import DefaultDict

from scripts import download_csv_files_if_missing
from scripts.csv_fields import SPECIAL_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.csv_files import route_stats_all

PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    download_csv_files_if_missing(route_stats_all)
    usecols = [f"{port}_{tag}" for tag in LEVELS for port in PORTS] + list(TAGS) 

    n_route = 0
    n_meh = 0
    counts = DefaultDict(int)
    for csv_file in route_stats_all:
        df = csv_file.read_w_default_config(usecols)
        n_route += len(df)

        df["total"] = sum((df[f"{port}_{tag}"] for tag in LEVELS for port in PORTS))
        df["total_meh"] = df["import_meh"] + df["export_meh"]
        df = df[df["total_meh"] > 0]
        n_meh += len(df)

        for tag in TAGS:
            d = df[df[tag] == df["total"]]
            count = len(d)
            counts[f"all_{tag}"] += count
            counts["all_same"] += count

            d = df[df[tag] == df["total_meh"]]
            count  = len(d)
            counts[f"meh_all_{tag}"] += count
            counts[f"meh_all_same"] += count

            d = df[df[tag] > 0]
            counts[f"some_{tag}"] += len(d)

        del df

    print(f"{n_route} routes in total, {n_meh} have special/whitelisted cases.")
    for tag in TAGS:
        percentage = counts[f"all_{tag}"] * 100 / n_route
        print(f"{counts[f"all_{tag}"]} all {tag}, {percentage:.1f}%.")
    percentage = counts["all_same"] * 100 / n_route
    print(f"{counts["all_same"]} all same status, {percentage:.1f}%.\n")

    for tag in TAGS:
        percentage = counts[f"meh_all_{tag}"] * 100 / n_route
        print(f"{counts[f"meh_all_{tag}"]} all {tag} among special/whitelisted cases, {percentage:.1f}%.")
    percentage = counts["meh_all_same"] * 100 / n_route
    print(f"{counts["meh_all_same"]} all same subtype among special/whitelisted case, {percentage:.1f}%.\n")

    for tag in TAGS:
        percentage = counts[f"some_{tag}"] * 100 / n_route
        print(f"{counts[f"some_{tag}"]} have {tag}, {percentage:.1f}%.")


if __name__ == "__main__":
    main()

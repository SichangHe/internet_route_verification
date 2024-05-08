"""Run at `scripts/` with `python3 -m scripts.stats.route_all_all_some`.
Takes ~12min.

Adopted from `as_all_all_some`.
"""

from typing import DefaultDict

from scripts import download_csv_files_if_missing
from scripts.csv_files import route_stats_all

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")
NON_SKIP_TAGS = ("ok", "meh", "err")


def main() -> None:
    download_csv_files_if_missing(route_stats_all)
    usecols = [f"{port}_{tag}" for tag in TAGS for port in PORTS]

    n_route = 0
    counts = DefaultDict(int)
    for csv_file in route_stats_all:
        df = csv_file.read_w_default_config(usecols)
        n_route += len(df)

        df["total"] = sum((df[f"{port}_{tag}"] for tag in TAGS for port in PORTS))
        df["total_non_skips"] = sum(
            (df[f"{port}_{tag}"] for tag in NON_SKIP_TAGS for port in PORTS)
        )
        for tag in TAGS:
            d = df[df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]]
            count = len(d)
            counts[f"all_{tag}"] += count
            counts["all_same"] += count

            d = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0]
            counts[f"some_{tag}"] += len(d)

        for tag in NON_SKIP_TAGS:
            d = df[df[f"import_{tag}"] + df[f"export_{tag}"] == df["total_non_skips"]]
            count = len(d)
            counts[f"all_{tag}_non_skips"] += count
            counts["all_same_non_skips"] += count

        for port in PORTS:
            df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))
            df[f"total_{port}_non_skips"] = sum(
                (df[f"{port}_{tag}"] for tag in NON_SKIP_TAGS)
            )

            for tag in TAGS:
                d = df[
                    (df[f"total_{port}"] != 0)
                    & (df[f"{port}_{tag}"] == df[f"total_{port}"])
                ]
                count = len(d)
                counts[f"all_same_{port}_{tag}"] += count
                counts[f"all_same_{port}"] += count

                d = df[df[f"{port}_{tag}"] > 0]
                counts[f"some_{port}_{tag}"] += len(d)

            for tag in NON_SKIP_TAGS:
                d = df[
                    (df[f"total_{port}_non_skips"] != 0)
                    & (df[f"{port}_{tag}"] == df[f"total_{port}_non_skips"])
                ]
                count = len(d)
                counts[f"all_same_{port}_{tag}_non_skips"] += count
                counts[f"all_same_{port}_non_skips"] += count

        del df

    print(f"{n_route} routes in total.")
    for tag in TAGS:
        count = counts[f"all_{tag}"]
        percentage = count * 100 / n_route
        print(f"{count} all {tag}, {percentage:.1f}%.")
    count = counts["all_same"]
    percentage = count * 100 / n_route
    print(f"{count} all same status, {percentage:.1f}%.\n")

    for tag in NON_SKIP_TAGS:
        count = counts[f"all_{tag}_non_skips"]
        percentage = count * 100 / n_route
        print(f"{count} all {tag}, excluding skips, {percentage:.1f}%.")
    count = counts["all_same_non_skips"]
    percentage = count * 100 / n_route
    print(f"{count} all same status, excluding skips, {percentage:.1f}%.\n")

    for tag in TAGS:
        count = counts[f"some_{tag}"]
        percentage = count * 100 / n_route
        print(f"{count} have {tag}, {percentage:.1f}%.")

    for port in PORTS:
        print()

        for tag in TAGS:
            count = counts[f"all_same_{port}_{tag}"]
            percentage = count / n_route * 100
            print(f"{count} all {tag} in {port}, {percentage:.1f}%.")
        count = counts[f"all_same_{port}"]
        percentage = count / n_route * 100
        print(f"{count} all same status in {port}, {percentage:.1f}%.\n")

        for tag in TAGS:
            count = counts[f"some_{port}_{tag}"]
            percentage = count / n_route * 100
            print(f"{count} have {tag} in {port}, {percentage:.1f}%.")

        for tag in NON_SKIP_TAGS:
            count = counts[f"all_same_{port}_{tag}_non_skips"]
            percentage = count / n_route * 100
            print(f"{count} all {tag} in {port}, excluding skips, {percentage:.1f}%.")


if __name__ == "__main__":
    main()

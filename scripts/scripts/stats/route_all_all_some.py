"""Run at `scripts/` with `python3 -m scripts.stats.route_all_all_some`.
Takes ~12min.

Adopted from `as_all_all_some`.
"""

from typing import DefaultDict

from scripts import download_csv_files_if_missing
from scripts.csv_files import route_stats_all

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def main() -> None:
    download_csv_files_if_missing(route_stats_all)
    usecols = [f"{port}_{tag}" for tag in TAGS for port in PORTS]

    n_route = 0
    counts = DefaultDict(int)
    for csv_file in route_stats_all:
        df = csv_file.read_w_default_config(usecols)
        n_route += len(df)

        df["total"] = sum((df[f"{port}_{tag}"] for tag in TAGS for port in PORTS))
        for tag in TAGS:
            d = df[
                df[f"import_{tag}"] + df[f"export_{tag}"] == df["total"]
            ]
            count = len(d)
            counts[f"all_{tag}"] += count
            counts["all_same"] += count

            d = df[df[f"import_{tag}"] + df[f"export_{tag}"] > 0]
            counts[f"some_{tag}"] += len(d)

        for port in PORTS:
            df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))

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

        del df

    print(f"{n_route} routes in total.")
    for tag in TAGS:
        percentage = counts[f"all_{tag}"] * 100 / n_route
        print(f"{counts[f"all_{tag}"]} all {tag}, {percentage:.1f}%.")
    percentage = counts["all_same"] * 100 / n_route
    print(f"{counts["all_same"]} all same status, {percentage:.1f}%.\n")


    for tag in TAGS:
        percentage = counts[f"some_{tag}"] * 100 / n_route
        print(f"{counts[f"some_{tag}"]} have {tag}, {percentage:.1f}%.")


    for port in PORTS:
        print()

        for tag in TAGS:
            percentage = counts[f"all_same_{port}_{tag}"] / n_route * 100
            print(
                f"{counts[f"all_same_{port}_{tag}"]} all {tag} in {port}, {percentage:.1f}%."
            )
        percentage = counts[f"all_same_{port}"] / n_route * 100
        print(f"{counts[f'all_same_{port}']} all same status in {port}, {percentage:.1f}%.\n")

        for tag in TAGS:
            percentage = counts[f"some_{port}_{tag}"] / n_route * 100
            print(
                f"{counts[f"some_{port}_{tag}"]} have {tag} in {port}, {percentage:.1f}%."
            )


if __name__ == "__main__":
    main()

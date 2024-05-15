"""Run at `scripts/` with `python3 -m scripts.stats.imports_exports`.

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
    print(f"{n_as:,} ASes in total.")

    for tag in TAGS:
        df[f"total_{tag}"] = sum((df[f"{port}_{tag}"] for port in PORTS))
    df["total"] = sum((df[f"total_{tag}"] for tag in TAGS))

    n_total = df["total"].sum()
    print(f"{n_total:,} total imports/exports.")
    for tag in TAGS:
        n_tag = df[f"total_{tag}"].sum()
        percentage = n_tag * 100.0 / n_total
        print(f"{n_tag:,} total {tag}, {percentage:.1f}%.")

    for port in PORTS:
        df[f"total_{port}"] = sum((df[f"{port}_{tag}"] for tag in TAGS))
        n_port = df[f"total_{port}"].sum()
        percentage = n_port * 100.0 / n_total
        print(f"\n{n_port:,} {port}, {percentage:.1f}%.")
        for tag in TAGS:
            n_tag = df[f"{port}_{tag}"].sum()
            percentage_total = n_tag * 100.0 / n_total
            percentage = n_tag * 100.0 / n_port
            print(
                f"{n_tag:,} {tag} in {port}, {percentage_total:.1f}% out of total, {percentage:.1f}% out of {port}."
            )


main() if __name__ == "__main__" else None

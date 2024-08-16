"""Run at `scripts/` as `python3 -m scripts.actions.find_relaxed`.

Find ASes with many `spec_import_customer` or `spec_export_customers`.
"""

from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")
CASES = ("spec_import_customer", "spec_export_customers")
MANY = 100


def find_aut_nums_w_many_relaxed_filters():
    df = as_stats_all_df(
        [f"{port}_{tag}" for tag in TAGS for port in PORTS] + ["aut_num"] + list(CASES)
    )
    df_raw = df  # For interactive use.
    _ = df_raw
    df = df[sum(df[tag] for tag in CASES) > 0]
    print(f"{len(df)} ASes have at least one of {CASES}.")
    df_relaxed = df
    aut_nums: dict[str, list[int]] = {}
    for tag in CASES:
        df = df_relaxed
        df = df[df[tag] > 0]
        print(f"\n{len(df)} ASes have `{tag}`s at all:\n{df}")
        df = df[df[tag] >= MANY]
        print(f"\n{len(df)} ASes at least {MANY} `{tag}`s:\n{df}")
        aut_nums[tag] = df.index.to_list()
    return aut_nums


def main():
    _ = find_aut_nums_w_many_relaxed_filters()


main() if __name__ == "__main__" else None

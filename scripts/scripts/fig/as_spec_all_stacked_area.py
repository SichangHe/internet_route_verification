"""Run at `scripts/` with `python3 -m scripts.fig.as_spec_all_stacked_area`.
"""

from concurrent import futures

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import CsvFile
from scripts.csv_files import as_stats_all
from scripts.fig import smart_sample

FILES = as_stats_all
PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")
TAGS = (
    "spec_export_customers",
    "spec_as_is_origin_but_no_route",
    "spec_as_set_contains_origin_but_no_route",
    "spec_import_from_neighbor",
    "spec_uphill",
    "spec_uphill_tier1",
    "spec_tier1_pair",
    "spec_import_peer_oifps",
    "spec_import_customer_oifps",
)


def read_as_stats(file: CsvFile):
    return pd.read_csv(
        file.path,
        dtype="uint",
        index_col="aut_num",
        usecols=[f"{port}_{level}" for port in PORTS for level in LEVELS]
        + list(TAGS)
        + ["aut_num"],
        engine="pyarrow",
    )


def plot():
    with futures.ProcessPoolExecutor() as executor:
        df = (
            pd.concat(executor.map(read_as_stats, FILES), copy=False)
            .groupby("aut_num")
            .sum(engine="pyarrow")
        )

    d = pd.DataFrame(
        {
            "total_spec": sum(df[tag] for tag in TAGS),
            "total_report": sum(
                df[f"{port}_{level}"] for port in PORTS for level in LEVELS
            ),
        }
    )
    d["spec_rate"] = d["total_spec"] / d["total_report"]
    d["%non_spec"] = 100.0 - (d["spec_rate"] * 100.0)
    for tag in TAGS:
        d[f"%{tag}"] = df[tag] / d["total_spec"] * 100.0 * d["spec_rate"]
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in TAGS] + ["%non_spec"],
        ascending=[False for _ in TAGS] + [True],
        ignore_index=True,
        inplace=True,
    )
    indexes, values = smart_sample(
        tuple(d[f"%{tag}"] for tag in TAGS),
        min_gap_frac=0.0003,
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=[
            "%Export Customer",
            "%AS Is Origin",
            "%as-set ∋ Origin",
            "%Import Neighbor",
            "%Uphill",
            "%Uphill Tier-1",
            # "%spec_import_peer_oifps",
            # "%spec_import_customer_oifps",
        ],
    )
    ax.set_xlabel("AS Ordered by Associated Special Case", fontsize=36)
    ax.set_ylabel(f"Percentage of Special Case", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", fontsize=36)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    with futures.ThreadPoolExecutor() as executor:
        executor.map(CsvFile.download_if_missing, FILES)

    fig, _, _ = plot()

    pdf_name = f"AS-all-special-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

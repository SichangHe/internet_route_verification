"""Run at `scripts/` with `python3 -m scripts.fig.as_unrec_stacked_area`."""

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts.csv_fields import UNRECORDED_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.csv_files import as_stats
from scripts.fig import smart_sample

FILE = as_stats
PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def plot() -> tuple[Figure, Axes, pd.DataFrame]:
    df = pd.read_csv(
        FILE.path,
        index_col="aut_num",
        usecols=[f"{port}_{level}" for port in PORTS for level in LEVELS]  # type: ignore
        + list(TAGS)
        + ["aut_num"],
    )

    d = pd.DataFrame(
        {
            "total_unrec": sum(df[tag] for tag in TAGS),
            "total_report": sum(
                df[f"{port}_{level}"] for port in PORTS for level in LEVELS
            ),
        }
    )
    d["unrec_rate"] = d["total_unrec"] / d["total_report"]
    d["%non_unrec"] = 100.0 - (d["unrec_rate"] * 100.0)
    for tag in TAGS:
        d[f"%{tag}"] = df[tag] / d["total_unrec"] * 100.0 * d["unrec_rate"]
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in TAGS] + ["%non_unrec"],
        ascending=[False for _ in TAGS] + [True],
        ignore_index=True,
        inplace=True,
    )
    indexes, values = smart_sample(
        tuple(d[f"%{tag}"] for tag in TAGS),  # type: ignore[reportArgumentType]
        min_gap_frac=0.0002,
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=[
            "%Import Rule",
            "%Export Rule",
            "%aut-num",
            "%as-set Route",
            "%∃as-set Route",
            "%as-set",
            "%AS Route",
            "%route-set",
            # "%peering-set"
            # "%filter-set"
        ],
    )
    ax.set_xlabel("ASes Ordered by Associated Unrecorded Cases", fontsize=36)
    ax.set_ylabel("Percentages of Unrecorded Cases", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", fontsize=32)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    FILE.download_if_missing()
    fig, _, _ = plot()

    pdf_name = "AS-unrec-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

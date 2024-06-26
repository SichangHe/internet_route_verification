"""Run at `scripts/` with `python3 -m scripts.fig.as_unrec_all_stacked_area`."""

import re

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts.csv_fields import MODIFIED_UNRECORDED_CASE_FIELDS as MODIFIED_TAGS
from scripts.csv_fields import MODIFIED_UNRECORDED_CASE_LABELS
from scripts.csv_fields import UNRECORDED_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.fig import smart_sample
from scripts.fig.colors import COLORS7
from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def plot() -> tuple[Figure, Axes, pd.DataFrame]:
    df = as_stats_all_df(
        [f"{port}_{level}" for port in PORTS for level in LEVELS]
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
    for tag in MODIFIED_TAGS:
        d[f"%{tag}"] = (
            sum(
                df[matching_tag]
                for matching_tag in TAGS
                if re.match(f"^{tag}$", matching_tag)
            )
            / d["total_unrec"]
            * 100.0
            * d["unrec_rate"]
        )
    d.dropna(inplace=True)

    d.sort_values(
        by=[f"%{tag}" for tag in MODIFIED_TAGS] + ["%non_unrec"],
        ascending=[False for _ in MODIFIED_TAGS] + [True],
        ignore_index=True,
        inplace=True,
    )
    indexes, values = smart_sample(
        tuple(d[f"%{tag}"] for tag in MODIFIED_TAGS),  # type: ignore
        min_gap_frac=0.0002,
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        colors=reversed(COLORS7),  # type: ignore[reportArgumentType]
        labels=MODIFIED_UNRECORDED_CASE_LABELS,
    )
    ax.fill_between([], [], [], label="Not Unrecorded", color="white")
    ax.set_xlabel(
        "ASes with Unrecorded Cases, Ordered by\nPrevalent Subtypes", fontsize=36
    )
    ax.set_ylabel("Percentage of Imports and\nExports per AS", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower left", fontsize=30)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    fig, _, _ = plot()

    pdf_name = "AS-all-unrec-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

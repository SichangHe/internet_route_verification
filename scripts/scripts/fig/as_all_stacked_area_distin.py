"""Run at `scripts/` with `python3 -m scripts.fig.as_all_stacked_area_distin`.

Adopted from `as_all_stacked_area`.
"""

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts.csv_fields import SAFELIST_REPORT_ITEM_FIELDS
from scripts.fig import smart_sample
from scripts.fig.colors import COLORS6
from scripts.fig.dataframes import as_stats_all_df

PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot() -> tuple[Figure, Axes, pd.DataFrame, pd.DataFrame]:
    df = as_stats_all_df(
        ["aut_num"]
        + [f"{port}_{tag}" for port in PORTS for tag in TAGS]
        + list(SAFELIST_REPORT_ITEM_FIELDS)
    )
    df["safelisted"] = sum(df[tag] for tag in SAFELIST_REPORT_ITEM_FIELDS)
    df["special"] = sum(df[f"{port}_meh"] for port in PORTS) - df["safelisted"]

    d = pd.DataFrame(
        {"total": sum(df[f"{port}_{tag}"] for tag in TAGS for port in PORTS)}
    )
    for tag in TAGS:
        d[f"%{tag}"] = sum(df[f"{port}_{tag}"] for port in PORTS) * 100 / d["total"]
    for tag in ("special", "safelisted"):
        d[f"%{tag}"] = df[tag] * 100 / d["total"]
    d.dropna(inplace=True)
    d.sort_values(
        by=[
            f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "special", "safelisted")
        ],
        ascending=[False, True, False, False, False, False],
        ignore_index=True,
        inplace=True,
    )

    indexes, values = smart_sample(
        tuple(
            d[f"%{tag}"]
            for tag in ("ok", "skip", "unrec", "special", "safelisted", "err")
        ),  # type: ignore[reportArgumentType]
        min_gap_frac=0.0003,
    )

    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        colors=COLORS6,  # type: ignore[reportArgumentType]
        labels=(
            "Verified",
            "Skipped",
            "Unrecorded",
            "Relaxed",
            "Safelisted",
            "Unverified",
        ),
    )
    ax.set_xlabel("ASes Ordered by Correctness", fontsize=36)
    ax.set_ylabel("Percentage of Imports and\nExports per AS", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", bbox_to_anchor=(0.6, 0.0), fontsize=32)

    # For checking.
    # fig.show()

    return fig, ax, df, d


def main():
    fig, _, _, _ = plot()

    pdf_name = "AS-all-distin-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

"""Run at `scripts/` with `python3 -m scripts.fig.route_spec_stacked_area`.

Adopted from `as_spec_stacked_area.py`.
"""

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts.csv_fields import SPECIAL_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.csv_files import route_stats
from scripts.fig import smart_sample

FILE = route_stats


def plot() -> tuple[Figure, Axes, pd.DataFrame]:
    df = pd.read_csv(FILE.path, dtype="uint16")

    d = pd.DataFrame({"total": sum(df[tag] for tag in TAGS)})
    for tag in TAGS:
        d[f"%{tag}"] = df[tag] / d["total"] * 100.0
    d.dropna(inplace=True)
    d = d.sort_values(
        by=[f"%{tag}" for tag in TAGS],
        ascending=False,
        ignore_index=True,
    )
    indexes, values = smart_sample(
        tuple(d[f"%{tag}"] for tag in TAGS), min_gap_frac=0.0002
    )

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=[f"%{tag}" for tag in TAGS],
    )
    ax.set_xlabel("Route", fontsize=16)
    ax.set_ylabel(f"Percentage of Special Case", fontsize=16)
    ax.tick_params(axis="both", labelsize=14)
    ax.grid()
    ax.legend(loc="lower center", fontsize=14)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    FILE.download_if_missing()
    fig, _, _ = plot()

    pdf_name = f"route-special-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

"""Mostly the same as `scripts/fig/route_all_stacked_area.py`,
but for the first hop of each route.

Run at `scripts/` with `python3 -m scripts.fig.route_first_hop_all_stacked_area`.
"""

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_files import (
    route_first_hop_all_export_stats,
    route_first_hop_all_import_stats,
    route_first_hop_all_total_stats,
)
from scripts.fig import VERIFICATION_STATUSES
from scripts.fig.route_all_stacked_area import counted_smart_sample

FILES = (
    route_first_hop_all_import_stats,
    route_first_hop_all_export_stats,
    route_first_hop_all_total_stats,
)
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def process_route_stats(file: CsvFile, y_label: str):
    df = pd.read_csv(file.path, engine="pyarrow")
    indexes, values = counted_smart_sample(
        tuple(df[f"%{tag}"] for tag in TAGS),  # type: ignore
        df["count"],  # type: ignore
    )

    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=VERIFICATION_STATUSES,
    )
    ax.set_xlabel("Routes (First Hop) Ordered by Correctness", fontsize=36)
    ax.set_ylabel(f"Percentages of {y_label} in Hops", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower left", fontsize=32)

    return fig, ax, df


def plot() -> tuple[dict[str, Figure], dict[str, Axes], dict[str, pd.DataFrame]]:
    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}

    for key, file, y_label in zip(
        ("import", "export", "exchange"),
        FILES,
        ("Imports", "Exports", "Imports/Exports\n"),
    ):
        fig, ax, df = process_route_stats(file, y_label)
        dfs[key], figs[key], axs[key] = df, fig, ax

    return figs, axs, dfs


def main():
    download_csv_files_if_missing(FILES)

    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"route-first-hop-all-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(12, 9)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

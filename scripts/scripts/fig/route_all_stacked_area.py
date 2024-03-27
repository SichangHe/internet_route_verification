"""Run at `scripts/` with `python3 -m scripts.fig.route_all_stacked_area`.
"""

from concurrent import futures

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import CsvFile
from scripts.csv_files import (
    route_all_export_stats,
    route_all_import_stats,
    route_all_total_stats,
)

FILES = (route_all_import_stats, route_all_export_stats, route_all_total_stats)
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def counted_smart_sample(same: tuple[pd.Series], counts: pd.Series):
    """Only sample the indexes of boundary values.
    All same (plural of "series") and `counts` should have the same length."""
    size = len(same[0])

    cumm_index = 0
    indexes: list[int] = []
    values = tuple([] for _ in same)
    for index in range(size):
        value = tuple(series[index] for series in same)
        count = counts[index]

        indexes.append(cumm_index)
        for vs, v in zip(values, value):
            vs.append(v)

        cumm_index += count
        if count > 1:
            indexes.append(cumm_index - 1)
            for vs, v in zip(values, value):
                vs.append(v)
    return indexes, values


def process_route_stats(file: CsvFile, y_label: str):
    df = pd.read_csv(file.path, engine="pyarrow")
    indexes, values = counted_smart_sample(
        tuple(df[f"%{tag}"] for tag in TAGS), df["count"]
    )

    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.stackplot(
        indexes,
        values,
        labels=("%OK", "%Skip", "%Unrec", "%Special", "%Error"),
    )
    ax.set_xlabel("Route Ordered by Correctness", fontsize=36)
    ax.set_ylabel(f"Percentage of {y_label}", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower left", fontsize=36)

    return fig, ax, df


def plot():
    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}

    for key, file, y_label in zip(
        ("import", "export", "exchange"), FILES, ("Import", "Export", "Import/Export")
    ):
        fig, ax, df = process_route_stats(file, y_label)
        dfs[key], figs[key], axs[key] = df, fig, ax

    return figs, axs, dfs


def main():
    with futures.ThreadPoolExecutor() as executor:
        executor.map(CsvFile.download_if_missing, FILES)

    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"route-all-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(12, 9)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

"""Run at `scripts/` with `python3 -m scripts.fig.as_pair_all_stacked_area`.
"""

from concurrent import futures

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import CsvFile
from scripts.csv_files import as_pair_stats_all
from scripts.fig import smart_sample

FILES = as_pair_stats_all
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def read_as_pair_stats(file: CsvFile):
    return pd.read_csv(
        file.path,
        index_col=["from", "to"],
        usecols=["from", "to"] + [f"{port}_{tag}" for port in PORTS for tag in TAGS],
        engine="pyarrow",
    )


def plot():
    with futures.ProcessPoolExecutor() as executor:
        df = (
            pd.concat(
                (d for d in executor.map(read_as_pair_stats, FILES) if len(d) > 0),
                copy=False,
            )
            .groupby(["from", "to"])
            .sum(engine="pyarrow")
        )

    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}
    for port in PORTS:
        d = pd.DataFrame({"total": sum(df[f"{port}_{tag}"] for tag in TAGS)})
        for tag in TAGS:
            d[f"%{tag}"] = df[f"{port}_{tag}"] / d["total"] * 100.0
        d.dropna(inplace=True)
        d.sort_values(
            by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
            ascending=[False, True, False, False, False],
            ignore_index=True,
            inplace=True,
        )
        dfs[port] = d
    d = pd.DataFrame(
        {"total": sum(df[f"{port}_{tag}"] for tag in TAGS for port in PORTS)}
    )
    for tag in TAGS:
        d[f"%{tag}"] = sum(df[f"{port}_{tag}"] for port in PORTS) / d["total"] * 100.0
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
        ascending=[False, True, False, False, False],
        ignore_index=True,
        inplace=True,
    )
    dfs["exchange"] = d
    for (key, d), y_label in zip(
        dfs.items(),
        ("Import", "Export", "Import/Export"),
    ):
        indexes, values = smart_sample(
            tuple(d[f"%{tag}"] for tag in TAGS), min_gap_frac=0.0002
        )

        fig, ax = plt.subplots(figsize=(16, 9))
        figs[key], axs[key] = fig, ax
        fig.tight_layout()
        ax.stackplot(
            indexes,
            values,
            labels=("%OK", "%Skip", "%Unrec", "%Special", "%Error"),
        )
        ax.set_xlabel("AS Pair Ordered by Correctness", fontsize=36)
        ax.set_ylabel(f"Percentage of {y_label}", fontsize=36)
        ax.tick_params(axis="both", labelsize=32)
        ax.grid()
        ax.legend(loc="best", fontsize=36)

    # For checking.
    # figs["import"].show()

    return figs, axs, dfs


def main():
    with futures.ThreadPoolExecutor() as executor:
        executor.map(CsvFile.download_if_missing, FILES)

    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"AS-pair-all-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(12, 9)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

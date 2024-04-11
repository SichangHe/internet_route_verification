"""Run at `scripts/` with `python3 -m scripts.fig.as_stacked_area`.
"""

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts.csv_files import as_stats
from scripts.fig import smart_sample

FILE = as_stats
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot() -> tuple[dict[str, Figure], dict[str, Axes], dict[str, pd.DataFrame]]:
    df = pd.read_csv(
        FILE.path,
        index_col="aut_num",
        usecols=["aut_num"] + [f"{port}_{tag}" for port in PORTS for tag in TAGS],  # type: ignore
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
        ("Imports", "Exports", "Imports/Exports\n"),
    ):
        indexes, values = smart_sample(
            tuple(d[f"%{tag}"] for tag in TAGS), min_gap_frac=0.0003  # type: ignore
        )

        fig, ax = plt.subplots(figsize=(16, 9))
        figs[key], axs[key] = fig, ax
        fig.tight_layout()
        ax.stackplot(
            indexes,
            values,
            labels=("%OK", "%Skip", "%Unrec", "%Special", "%Error"),
        )
        ax.set_xlabel("ASes Ordered by Correctness", fontsize=36)
        ax.set_ylabel(f"Percentages of {y_label} in Routes", fontsize=36)
        ax.tick_params(axis="both", labelsize=32)
        ax.grid()
        ax.legend(loc="lower center", fontsize=32)

    # For checking.
    # figs["import"].show()

    return figs, axs, dfs


def main():
    FILE.download_if_missing()
    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"AS-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(12, 9)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

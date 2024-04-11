"""Run at `scripts/` with `python3 -m scripts.fig.as_spec_all_stacked_area`.
"""

from concurrent import futures

import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import CsvFile, download_csv_files_if_missing
from scripts.csv_fields import SPECIAL_CASE_REPORT_ITEM_FIELDS as TAGS
from scripts.csv_files import as_stats_all
from scripts.fig import smart_sample

FILES = as_stats_all
PORTS = ("import", "export")
LEVELS = ("ok", "skip", "unrec", "meh", "err")


def read_as_stats(file: CsvFile):
    return pd.read_csv(
        file.path,
        index_col="aut_num",
        usecols=[f"{port}_{level}" for port in PORTS for level in LEVELS]  # type: ignore
        + list(TAGS)
        + ["aut_num"],
        engine="pyarrow",
    )


def plot() -> tuple[Figure, Axes, pd.DataFrame]:
    with futures.ProcessPoolExecutor() as executor:
        df = (
            pd.concat(
                (d for d in executor.map(read_as_stats, FILES) if len(d) > 0),
                copy=False,
            )
            .groupby("aut_num")
            .sum(engine="pyarrow")  # type: ignore
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
        tuple(d[f"%{tag}"] for tag in TAGS),  # type: ignore
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
            # FIXME: Should be changed.
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
    ax.set_xlabel("ASes Ordered by Associated Special Cases", fontsize=36)
    ax.set_ylabel(f"Percentages of Special Cases", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", fontsize=32)

    # For checking.
    # fig.show()

    return fig, ax, d


def main():
    download_csv_files_if_missing(FILES)

    fig, _, _ = plot()

    pdf_name = f"AS-all-special-case-percentages-stacked-area.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

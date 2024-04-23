"""Run at `scripts/` with `python3 -m scripts.fig.as_rules_cdf`.
"""

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure

from scripts import download_csv_files_if_missing
from scripts.csv_files import as_compatible_with_bgpq3, as_neighbors_vs_rules

TIER1S = set(
    (
        174,
        209,
        286,
        # 701,
        1239,
        1299,
        # 2828,
        2914,
        3257,
        3320,
        3356,
        3491,
        5511,
        # 6453,
        # 6461,
        6762,
        6830,
        # 7018,
        12956,
    )
)
"""ASes without aut-num objects are commented out."""


def plot() -> tuple[Figure, Axes]:
    df_raw = pd.read_csv(as_neighbors_vs_rules.path)

    # Remove ASes not in IRR.
    df = df_raw.drop(df_raw[df_raw["import"] == -1].index)  # type: ignore
    df["rules"] = df["import"] + df["export"]

    # CCDF plotting reference: `matplotlib/axes/_axes.py`.
    cdf_data = np.array(df["rules"])
    cdf_order = np.argsort(cdf_data)
    cdf_data = cdf_data[cdf_order]
    aut_num_sorted = np.asarray(df["aut_num"])[cdf_order]
    cum_weights = 1 - ((1 + np.arange(len(cdf_data))) / len(cdf_data))

    tier1labels, tier1cdf_data, tier1cum_weights = [], [], []
    for aut_num, n_rules, cum_weight in zip(aut_num_sorted, cdf_data, cum_weights):
        if aut_num in TIER1S:
            tier1labels.append(f"AS{aut_num}")
            tier1cdf_data.append(n_rules)
            tier1cum_weights.append(cum_weight)

    cdf_data = np.concatenate((cdf_data, np.asarray((cdf_data[-1],))))
    cum_weights = np.concatenate((np.asarray((1,)), cum_weights))

    bgpq3_compatible = pd.read_csv(as_compatible_with_bgpq3.path)[
        "as_compatible_w_bgpq3"
    ]
    assert isinstance(bgpq3_compatible, pd.Series)
    df_bgpq3_compatible = df[df["aut_num"].isin(bgpq3_compatible) & (df["rules"] > 0)]
    df_incompatible = df[~df["aut_num"].isin(bgpq3_compatible) & (df["rules"] > 0)]

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    ax.plot(
        cdf_data,
        cum_weights,
        drawstyle="steps-pre",
        linewidth=4,
        label="All aut-num Objects",
        zorder=5,
    )
    ax.ecdf(
        df_bgpq3_compatible["rules"],
        complementary=True,
        linewidth=2,
        label="BGPq3-Compatible",
    )
    ax.ecdf(
        df_incompatible["rules"],
        complementary=True,
        linewidth=2,
        label="Incompatible",
    )

    ax.scatter(
        tier1cdf_data,
        tier1cum_weights,
        c="red",
        s=400,
        marker="x",
        linewidth=4,
        label="Tier-1 ASes",
        zorder=10,
    )
    for label, x, y in zip(tier1labels, tier1cdf_data, tier1cum_weights):
        ax.annotate(
            label,
            (x, y),
            textcoords="offset points",
            xytext=(-35, -30),  # Modify this to move text around.
            ha="center",
            zorder=100,
        )
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("Number of Import/Export Rules", fontsize=36)
    ax.set_ylabel("Complementary Cumulative\nFraction of ASes", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend()
    # ax.legend(loc="best", fontsize=36)

    # For checking.
    # fig.show()

    return fig, ax


def main():
    download_csv_files_if_missing((as_neighbors_vs_rules, as_compatible_with_bgpq3))
    fig, _ = plot()

    pdf_name = "CDF-AS-rules.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

"""Run at `scripts/` with `python3 -m scripts.fig.as_bgpq3_rules_cdf`.

Adopted from `as_rules_cdf`.
"""

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from numpy.typing import NDArray

from scripts.csv_files import bgpq3_compatible_rules
from scripts.fig.as_rules_cdf import GIANTS, TIER1S
from scripts.fig.colors import hue_grayscale_to_srgb


def plot() -> tuple[Figure, Axes]:
    df = pd.read_csv(bgpq3_compatible_rules.path)

    df["rules"] = df["import"] + df["export"]
    df["simple_rules"] = df["simple_import"] + df["simple_export"]

    # CCDF plotting reference: `matplotlib/axes/_axes.py`.
    cdf_data: NDArray[np.floating] = np.asarray(df["rules"])
    cdf_order = np.argsort(cdf_data)
    cdf_data = cdf_data[cdf_order]
    aut_num_sorted: NDArray[np.integer] = np.asarray(df["aut_num"])[cdf_order]
    cum_weights = 1 - ((1 + np.arange(len(cdf_data))) / len(cdf_data))

    # Tier-1 and large cloud providers scatter plot data.
    tier1labels: list[str] = []
    tier1cdf_data: list[int] = []
    tier1cum_weights: list[float] = []

    giant_labels: list[str] = []
    giant_cdf_data: list[int] = []

    giant_cum_weights: list[float] = []
    tier1s_wo_aut_num = {aut_num for aut_num in TIER1S}
    tier1s_w0rule: set[int] = set()
    giants_wo_aut_num = {aut_num for aut_num in GIANTS}
    giants_w0rule: set[int] = set()
    for aut_num, n_rules, cum_weight in zip(aut_num_sorted, cdf_data, cum_weights):
        if aut_num in TIER1S:
            if n_rules == 0:
                tier1s_w0rule.add(aut_num)
            tier1s_wo_aut_num.remove(aut_num)
            tier1labels.append(f"AS{aut_num}")
            tier1cdf_data.append(n_rules)
            tier1cum_weights.append(cum_weight)
        elif aut_num in GIANTS:
            giants_wo_aut_num.remove(aut_num)
            if n_rules == 0:
                giants_w0rule.add(aut_num)
            label = f"AS{aut_num} ({GIANTS[aut_num]})"
            try:
                index = giant_cdf_data.index(n_rules)
                giant_labels[index] += f", {label}"
            except ValueError:
                giant_labels.append(label)
                giant_cdf_data.append(n_rules)
                giant_cum_weights.append(cum_weight)

    print(
        f"""Tier-1 ASes without aut-num: {tier1s_wo_aut_num}.
Tier-1 ASes with 0 rule: {tier1s_w0rule}.
Large cloud providers without aut-num: {giants_wo_aut_num}.
Large cloud providers with 0 rule: {giants_w0rule}."""
    )

    # BGPq3-compatible/incompatible ASes CDF data.
    n_wo_rule = len(df[df["simple_rules"] == 0])
    percentage = n_wo_rule * 100 / len(df)
    print(
        f"{n_wo_rule} ({percentage:.1f}%) aut-num objects have 0 BGPq3-compatible rules."
    )

    compatible_cdf_data = np.asarray(df["simple_rules"])
    compatible_cdf_order = np.argsort(compatible_cdf_data)
    compatible_cdf_data = compatible_cdf_data[compatible_cdf_order]
    compatible_cum_weights = (
        len(compatible_cdf_data) - (1 + np.arange(len(compatible_cdf_data)))
    ) / len(cdf_data)
    compatible_cdf_data = np.concatenate(
        (compatible_cdf_data, np.asarray((compatible_cdf_data[-1],)))
    )
    compatible_cum_weights = np.concatenate((np.asarray((1,)), compatible_cum_weights))

    cdf_data = np.concatenate((cdf_data, np.asarray((cdf_data[-1],))))
    cum_weights = np.concatenate((np.asarray((1,)), cum_weights))

    fig: Figure
    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()

    # CCDF plots.
    ax.plot(
        cdf_data,
        cum_weights,
        drawstyle="steps-pre",
        linewidth=4,
        color=hue_grayscale_to_srgb(60, 0.92) + (0.9,),
        linestyle="-.",
        label="All Rules",
        zorder=5,
    )
    ax.plot(
        compatible_cdf_data,
        compatible_cum_weights,
        drawstyle="steps-pre",
        linewidth=4,
        color=hue_grayscale_to_srgb(240, 0.2),
        label="BGPq3-Compatible Rules",
    )

    # Tier-1 and large cloud providers scatter plots.
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
            xytext=(8, -30),  # Modify this to move text around.
            ha="right",
            zorder=100,
        )

    ax.scatter(  # Dummy plot to add legend entry.
        [],
        [],
        c="green",
        s=400,
        linewidth=4,
        marker=r"$\leftarrow$",
        label="Large Cloud Providers",
    )
    for label, x, y in zip(giant_labels, giant_cdf_data, giant_cum_weights):
        ax.annotate(
            "",
            (x, y),
            textcoords="offset points",
            xytext=(30, 10),  # Modify this to move tail around.
            arrowprops={
                "width": 4,
                "headwidth": 16,
                "facecolor": "green",
                "edgecolor": "green",
            },
            zorder=12,
        )
        ax.annotate(
            label,
            (x, y),
            textcoords="offset points",
            xytext=(35, 0),  # Modify this to move text around.
            zorder=100,
        )

    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel(
        "Number of $import$ and $export$ Rules per $aut$-$num$ Object", fontsize=36
    )
    ax.set_ylabel(
        "Complementary Cumulative\nFraction of $aut$-$num$ Objects",
        fontsize=36,
    )
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(fontsize=30, loc="lower left")

    # For checking.
    # fig.show()

    return fig, ax


def main():
    bgpq3_compatible_rules.download_if_missing()
    fig, _ = plot()

    pdf_name = "CDF-AS-bgpq3-rules.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()

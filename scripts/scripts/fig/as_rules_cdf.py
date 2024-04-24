"""Run at `scripts/` with `python3 -m scripts.fig.as_rules_cdf`.
"""

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from pandas.core.frame import com

from scripts import download_csv_files_if_missing
from scripts.csv_files import as_compatible_with_bgpq3, as_neighbors_vs_rules

TIER1S = {
    174,
    209,
    286,
    701,
    1239,
    1299,
    2828,
    2914,
    3257,
    3320,
    3356,
    3491,
    5511,
    6453,
    6461,
    6762,
    6830,
    7018,
    12956,
}

GIANTS = {
    8075: "Microsoft",
    15169: "Google",
    16509: "AWS",
    32934: "Facebook",
    36351: "Softlayer/IBM",
    13335: "Cloudflare",
}


def plot() -> tuple[Figure, Axes]:
    df_raw = pd.read_csv(as_neighbors_vs_rules.path)

    # Remove ASes not in IRR.
    df = df_raw.drop(df_raw[df_raw["import"] == -1].index)  # type: ignore
    df["rules"] = df["import"] + df["export"]

    # CCDF plotting reference: `matplotlib/axes/_axes.py`.
    cdf_data = np.asarray(df["rules"])
    cdf_order = np.argsort(cdf_data)
    cdf_data = cdf_data[cdf_order]
    aut_num_sorted = np.asarray(df["aut_num"])[cdf_order]
    cum_weights = 1 - ((1 + np.arange(len(cdf_data))) / len(cdf_data))

    # Tier-1 and large cloud providers scatter plot data.
    tier1labels, tier1cdf_data, tier1cum_weights = [], [], []
    giant_labels, giant_cdf_data, giant_cum_weights = [], [], []
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
    bgpq3_compatible = pd.read_csv(as_compatible_with_bgpq3.path)[
        "as_compatible_w_bgpq3"
    ]
    assert isinstance(bgpq3_compatible, pd.Series)

    df_bgpq3_compatible = df[df["aut_num"].isin(bgpq3_compatible) & (df["rules"] > 0)]
    compatible_cdf_data = np.asarray(df_bgpq3_compatible["rules"])
    compatible_cdf_order = np.argsort(compatible_cdf_data)
    compatible_cdf_data = compatible_cdf_data[compatible_cdf_order]
    compatible_cum_weights = (
        len(compatible_cdf_data) - (1 + np.arange(len(compatible_cdf_data)))
    ) / len(cdf_data)
    compatible_cdf_data = np.concatenate(
        (compatible_cdf_data, np.asarray((compatible_cdf_data[-1],)))
    )
    compatible_cum_weights = np.concatenate((np.asarray((1,)), compatible_cum_weights))

    df_incompatible = df[~df["aut_num"].isin(bgpq3_compatible) & (df["rules"] > 0)]
    incomp_cdf_data = np.asarray(df_incompatible["rules"])
    incomp_cdf_order = np.argsort(incomp_cdf_data)
    incomp_cdf_data = incomp_cdf_data[incomp_cdf_order]
    incomp_cum_weights = (
        len(incomp_cdf_data) - (1 + np.arange(len(incomp_cdf_data)))
    ) / len(cdf_data)
    incomp_cdf_data = np.concatenate(
        (incomp_cdf_data, np.asarray((incomp_cdf_data[-1],)))
    )
    incomp_cum_weights = np.concatenate((np.asarray((1,)), incomp_cum_weights))

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
        label="All aut-num Objects",
        zorder=5,
    )
    ax.plot(
        compatible_cdf_data,
        compatible_cum_weights,
        drawstyle="steps-pre",
        linewidth=2,
        label="BGPq3-Compatible",
    )
    ax.plot(
        incomp_cdf_data,
        incomp_cum_weights,
        drawstyle="steps-pre",
        linewidth=2,
        label="BGPq3-Incompatible",
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

    ax.scatter(
        giant_cdf_data,
        giant_cum_weights,
        c="purple",
        s=200,
        marker="^",
        linewidth=2,
        label="Large Cloud Providers",
        zorder=12,
    )
    for label, x, y in zip(giant_labels, giant_cdf_data, giant_cum_weights):
        ax.annotate(
            label,
            (x, y),
            c="purple",
            textcoords="offset points",
            xytext=(-8, 10),  # Modify this to move text around.
            ha="left",
            zorder=100,
        )

    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("Number of Import/Export Rules", fontsize=36)
    ax.set_ylabel("Complementary Cumulative\nFraction of ASes", fontsize=36)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(fontsize=32)

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

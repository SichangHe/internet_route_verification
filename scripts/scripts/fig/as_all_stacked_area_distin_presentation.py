"""Run at `scripts/` with `python3 -m scripts.fig.as_all_stacked_area_distin_presentation`.

Adopted from `as_all_stacked_area_distin`.
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


def main():
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

    fig: Figure
    ax: Axes
    labels = (
        "Verified",
        "Skipped",
        "Unrecorded",
        "Relaxed",
        "Safelisted",
        "Unverified",
    )
    # Legend-only.
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()

    ax.stackplot(
        indexes,
        tuple([0 for _ in value] for value in values),
        colors=COLORS6,  # type: ignore[reportArgumentType]
        labels=labels,
    )
    ax.set_ylim(0, 100)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", bbox_to_anchor=(0.6, 0.0), fontsize=32)

    fig.savefig(
        "AS-all-distin-percentages-stacked-area-legend.png",
        bbox_inches="tight",
        dpi=300,
    )

    def write_highlighted(highlight_labels: tuple[str, ...]):
        fig, ax = plt.subplots(figsize=(16, 9))
        fig.tight_layout()

        ax.stackplot(
            indexes,
            values,
            colors=[
                color if label in highlight_labels else color + (0.25,)
                for color, label in zip(COLORS6, labels)
            ],
            labels=labels,
        )
        ax.tick_params(axis="both", labelsize=32)
        ax.grid()
        ax.legend(loc="lower center", bbox_to_anchor=(0.6, 0.0), fontsize=32)

        suffix = "-".join((label[:4].lower() for label in highlight_labels))
        fig.savefig(
            f"AS-all-distin-percentages-stacked-area-{suffix}.png",
            bbox_inches="tight",
            dpi=300,
        )

    write_highlighted(("Skipped",))
    write_highlighted(("Unrecorded",))
    write_highlighted(("Safelisted",))
    write_highlighted(("Unrecorded", "Safelisted"))
    write_highlighted(("Relaxed",))
    write_highlighted(("Relaxed", "Unverified"))
    write_highlighted(("Unverified",))
    write_highlighted(("Unrecorded", "Relaxed", "Safelisted", "Unverified"))
    write_highlighted(("Verified", "Relaxed", "Safelisted", "Unverified"))

    # Full plot.
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
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    ax.legend(loc="lower center", bbox_to_anchor=(0.6, 0.0), fontsize=32)

    fig.savefig(
        "AS-all-distin-percentages-stacked-area.png", bbox_inches="tight", dpi=300
    )


if __name__ == "__main__":
    main()

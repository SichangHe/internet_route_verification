"""Run at `scripts/` with `python3 -m scripts.fig.last_modified`."""

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.axes import Axes

from scripts.csv_files import last_modified
from scripts.fig.colors import COLORS6

FILE = last_modified


def main():
    FILE.download_if_missing()
    df = pd.read_csv(
        FILE.path,
        sep="|",
        parse_dates=["last_modified"],
    )
    n_object = len(df)
    print(f"{n_object} RPSL objects.")

    ax: Axes
    fig, ax = plt.subplots(figsize=(16, 9))
    fig.tight_layout()
    index = 0

    df_class = df[(df["class"] == "route") | (df["class"] == "route6")]
    df_class_w_last_modified = df_class.dropna(subset=["last_modified"])  # type: ignore[reportCallIssue]

    percentage = len(df_class_w_last_modified) * 100 / len(df_class)
    print(
        f"\n{len(df_class_w_last_modified)} route[6] objects out of \
{len(df_class)} have last_modified ({percentage:.1f}%)."
    )
    print(df_class_w_last_modified.describe())

    ecdf(
        ax,
        df_class_w_last_modified["last_modified"],
        label="route[6]",
        linewidth=4,
        color=COLORS6[index],
    )

    classes = ["route-set", "as-set", "filter-set", "peering-set", "aut-num"]
    for class_name in classes:
        index += 1
        df_class = df[df["class"] == class_name]
        df_class_w_last_modified = df_class.dropna(subset=["last_modified"])  # type: ignore[reportCallIssue]

        percentage = len(df_class_w_last_modified) * 100 / len(df_class)
        print(
            f"\n{len(df_class_w_last_modified)} {class_name} objects out of \
{len(df_class)} have last_modified ({percentage:.1f}%)."
        )
        print(df_class_w_last_modified.describe())

        ecdf(
            ax,
            df_class_w_last_modified["last_modified"],
            label=class_name,
            linewidth=4,
            color=COLORS6[index],
        )

    ax.set_xlabel("Last Modified Datetime", fontsize=36)
    ax.set_ylabel("Cumulative Fraction of\nRPSL Objects", fontsize=36)
    ax.legend(fontsize=32)
    ax.tick_params(axis="both", labelsize=32)
    ax.grid()
    # fig.show() # For interactive use.

    pdf_name = "CDF-last-modified.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(12, 9)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


def ecdf(
    ax,
    x,
    weights=None,
    *,
    complementary=False,
    orientation="vertical",
    compress=False,
    **kwargs,
):
    """
    Copied from `matplotlib/axes/_axes.py` to work around NumPy NaN crashing.

    Compute and plot the empirical cumulative distribution function of *x*.

    .. versionadded:: 3.8

    Parameters
    ----------
    x : 1d array-like
        The input data.  Infinite entries are kept (and move the relevant
        end of the ecdf from 0/1), but NaNs and masked values are errors.

    weights : 1d array-like or None, default: None
        The weights of the entries; must have the same shape as *x*.
        Weights corresponding to NaN data points are dropped, and then the
        remaining weights are normalized to sum to 1.  If unset, all
        entries have the same weight.

    complementary : bool, default: False
        Whether to plot a cumulative distribution function, which increases
        from 0 to 1 (the default), or a complementary cumulative
        distribution function, which decreases from 1 to 0.

    orientation : {"vertical", "horizontal"}, default: "vertical"
        Whether the entries are plotted along the x-axis ("vertical", the
        default) or the y-axis ("horizontal").  This parameter takes the
        same values as in `~.Axes.hist`.

    compress : bool, default: False
        Whether multiple entries with the same values are grouped together
        (with a summed weight) before plotting.  This is mainly useful if
        *x* contains many identical data points, to decrease the rendering
        complexity of the plot. If *x* contains no duplicate points, this
        has no effect and just uses some time and memory.

    Other Parameters
    ----------------
    data : indexable object, optional
        DATA_PARAMETER_PLACEHOLDER

    **kwargs
        Keyword arguments control the `.Line2D` properties:

        %(Line2D:kwdoc)s

    Returns
    -------
    `.Line2D`

    Notes
    -----
    The ecdf plot can be thought of as a cumulative histogram with one bin
    per data entry; i.e. it reports on the entire dataset without any
    arbitrary binning.

    If *x* contains NaNs or masked entries, either remove them first from
    the array (if they should not taken into account), or replace them by
    -inf or +inf (if they should be sorted at the beginning or the end of
    the array).
    """
    x = np.asarray(x)
    argsort = np.argsort(x)
    x = x[argsort]
    if weights is None:
        # Ensure that we end at exactly 1, avoiding floating point errors.
        cum_weights = (1 + np.arange(len(x))) / len(x)
    else:
        weights = np.take(weights, argsort)  # Reorder weights like we reordered x.
        cum_weights = np.cumsum(weights / np.sum(weights))
    if compress:
        # Get indices of unique x values.
        compress_idxs = [0, *(x[:-1] != x[1:]).nonzero()[0] + 1]
        x = x[compress_idxs]
        cum_weights = cum_weights[compress_idxs]
    if orientation == "vertical":
        if not complementary:
            (line,) = ax.plot(
                [x[0], *x], [0, *cum_weights], drawstyle="steps-post", **kwargs
            )
        else:
            (line,) = ax.plot(
                [*x, x[-1]], [1, *1 - cum_weights], drawstyle="steps-pre", **kwargs
            )
        line.sticky_edges.y[:] = [0, 1]
    else:  # orientation == "horizontal":
        if not complementary:
            (line,) = ax.plot(
                [0, *cum_weights], [x[0], *x], drawstyle="steps-pre", **kwargs
            )
        else:
            (line,) = ax.plot(
                [1, *1 - cum_weights], [*x, x[-1]], drawstyle="steps-post", **kwargs
            )
        line.sticky_edges.x[:] = [0, 1]
    return line


if __name__ == "__main__":
    main()

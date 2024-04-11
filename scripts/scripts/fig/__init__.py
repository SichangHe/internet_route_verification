import matplotlib.pyplot as plt
import pandas as pd

plt.rcParams["axes.xmargin"] = 0
plt.rcParams["axes.ymargin"] = 0
plt.rcParams["font.size"] = 24
plt.rcParams["pdf.fonttype"] = 42
plt.rcParams["ps.fonttype"] = 42


def smart_sample(same: tuple[pd.Series, ...], min_gap_frac: float = 0):
    """Only sample the indexes of boundary values.
    All same (plural of "series") should have the same length."""
    size = len(same[0])
    max_index = size - 1
    min_gap = size * min_gap_frac

    indexes: list[int] = []
    values = tuple([] for _ in same)
    old_index = None
    old_value = None
    retaining = False
    for index in range(size):
        value = tuple(series[index] for series in same)

        within_gap = old_index is not None and index - old_index < min_gap
        no_change = value == old_value
        if index != max_index and (no_change or within_gap):
            retaining = True
            continue

        if retaining:
            assert old_value is not None
            indexes.append(index - 1)
            for vs, v in zip(values, old_value):
                vs.append(v)
            retaining = False

        old_index = index
        old_value = value
        indexes.append(index)
        for vs, v in zip(values, value):
            vs.append(v)
    return indexes, values

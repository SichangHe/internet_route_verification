"""Run at `scripts/` with `python3 -m scripts.stats.object_appearance_table`.
"""

import pandas as pd

from scripts import CsvFile
from scripts.csv_files import (
    as_num_appearances_in_rules,
    as_set_appearances_in_rules,
    filter_set_appearances_in_rules,
    peering_set_appearances_in_rules,
    route_set_appearances_in_rules,
)

FILES = (
    as_num_appearances_in_rules,
    as_set_appearances_in_rules,
    route_set_appearances_in_rules,
    peering_set_appearances_in_rules,
    filter_set_appearances_in_rules,
)
CLASSES = (r"\autnum", r"\asset", r"\routeset", r"\peeringset", r"\filterset")


def apperance_stats(file: CsvFile):
    file.download_if_missing()
    raw_df = pd.read_csv(file.path)

    df = raw_df[raw_df["recorded"]]
    defined = len(df)
    overall = len(df[df["import_overall"] + df["export_overall"] > 0])
    peering = len(df[df["import_peering"] + df["export_peering"] > 0])
    filter = len(df[df["import_filter"] + df["export_filter"] > 0])

    return [
        f"{defined}",
        f"{overall}",
        f"{overall*100/defined:.2f}",
        f"{peering}",
        f"{peering*100/defined:.2f}",
        f"{filter}",
        f"{filter*100/defined:.2f}",
    ]


def main():
    header = [
        "Class",
        "Defined",
        "Overall",
        r"\%Overall",
        r"\peering",
        r"\%\peering",
        r"\filter",
        r"\%\filter",
    ]
    columns = [header] + [
        [closs] + apperance_stats(file) for file, closs in zip(FILES, CLASSES)
    ]

    lengths = [max(len(element) for element in column) for column in columns]
    padded_columns = [
        [f"{element:<{length}}" for element in column]
        for column, length in zip(columns, lengths)
    ]
    rows = zip(*padded_columns)
    row_strs = (f" & ".join(row) + r" \\" for row in rows)
    print("\n".join(row_strs))


if __name__ == "__main__":
    main()

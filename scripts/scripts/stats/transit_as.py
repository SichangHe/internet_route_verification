"""Transit AS stats <https://github.com/SichangHe/internet_route_verification/issues/134>.
Run as `python3 -m scripts.stats.transit_as`.
"""

import pandas as pd

from scripts.csv_files import transit_as_stats

FILE = transit_as_stats
FIELDS = [
    "as_num",
    "import_provider",
    "import_peer",
    "import_customer",
    "import_other",
    "import_peering_provider",
    "import_filter_provider",
    "import_peering_peer",
    "import_filter_peer",
    "import_peering_customer",
    "import_filter_customer",
    "import_peering_other",
    "import_filter_other",
    "export_provider",
    "export_peer",
    "export_customer",
    "export_other",
    "export_self",
    "export_peering_provider",
    "export_filter_provider",
    "export_peering_peer",
    "export_filter_peer",
    "export_peering_customer",
    "export_filter_customer",
    "export_peering_other",
    "export_filter_other",
    "export_peering_self",
    "export_filter_self",
]


def main() -> None:
    FILE.download_if_missing()
    df = pd.read_csv(FILE.path)
    n_transit_as = len(df)

    df = df[df[FIELDS[1]] != -1]
    n_recorded_transit_as = len(df)
    print(
        f"{n_transit_as} transit AS in total, {n_recorded_transit_as} with recorded aut-num object ({n_recorded_transit_as * 100 / n_transit_as:2f}%)"
    )

    w_export_filter_self = df[df["export_filter_self"] > 0]
    n_w_export_filter_self = len(w_export_filter_self)
    print(
        f"{n_w_export_filter_self} with at least one export rule where they appear in the filter ({n_w_export_filter_self * 100 / n_recorded_transit_as:2f}%)"
    )


main() if __name__ == "__main__" else None

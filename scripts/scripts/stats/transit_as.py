"""Transit AS stats <https://github.com/SichangHe/internet_route_verification/issues/134>.
Run as `python3 -m scripts.stats.transit_as`.
"""

from typing import Final

import pandas as pd

from scripts.csv_files import transit_as_stats

FILE = transit_as_stats
FIELDS = [
    "as_num",
    "import_provider",
    "import_peer",
    "import_customer",
    "import_other",
    "import_both_provider",
    "import_both_peer",
    "import_both_customer",
    "import_both_other",
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
    df_raw: Final = pd.read_csv(FILE.path)
    df = df_raw
    n_transit_as = len(df)

    recorded_transit_as: Final = df[df[FIELDS[1]] != -1]
    df = recorded_transit_as
    n_recorded_transit_as = len(recorded_transit_as)
    print(
        f"{n_transit_as} transit AS in total, {n_recorded_transit_as} with recorded aut-num object ({n_recorded_transit_as * 100 / n_transit_as:2f}%)"
    )

    w_export_filter_self: Final = df[df["export_filter_self"] > 0]
    n_w_export_filter_self = len(w_export_filter_self)
    print(
        f"{n_w_export_filter_self} with at least one export rule where they appear in the filter ({n_w_export_filter_self * 100 / n_recorded_transit_as:2f}%)"
    )

    w_in_both: Final = df[
        (df["import_both_customer"] > 0)
        | (df["import_both_provider"] > 0)
        | (df["import_both_peer"] > 0)
        | (df["import_both_other"] > 0)
    ]
    n_w_in_both = len(w_in_both)
    print(
        f"{n_w_in_both} with AS in both peering and filter of the same import rules ({n_w_in_both * 100 / n_recorded_transit_as:2f}%)"
    )

    customer_in_both: Final = df[df["import_both_customer"] > 0]
    n_customer_in_both = len(customer_in_both)
    print(
        f"{n_customer_in_both} with customers in both peering and filter of the same import rules ({n_customer_in_both * 100 / n_recorded_transit_as:2f}%)"
    )

    only_provider_policies: Final = df[
        (df["import_peer"] == 0)
        & (df["import_customer"] == 0)
        & (df["import_other"] == 0)
        & (df["export_peer"] == 0)
        & (df["export_customer"] == 0)
        & (df["export_other"] == 0)
        & ((df["import_provider"] > 0) | (df["export_provider"] > 0))
    ]
    n_only_provider_policies = len(only_provider_policies)
    print(
        f"{n_only_provider_policies} only specify policies for providers ({n_only_provider_policies * 100 / n_recorded_transit_as:2f}%)"
    )


main() if __name__ == "__main__" else None

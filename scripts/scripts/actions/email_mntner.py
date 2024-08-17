"""Run at `scripts/` as `python3 -m scripts.actions.email_mntner`.

Email maintainers of ASes with many `spec_import_customer` or `spec_export_customers`.
"""

import json
import os
import subprocess
from dataclasses import dataclass
from typing import DefaultDict

from scripts.csv_files import relaxed_filter_as_info

relaxed_filter_tech_c_emails_path = "relaxed_filter_tech_c_emails.csv"
SPECIAL_WHOIS_ADDRESS = {
    "ARIN": "rr.arin.net",
    "LEVEL3": "rr.Level3.net",
    "NTTCOM": "rr.ntt.net",
    "JPIRR": "jpirr.nic.ad.jp",
    "TC": "whois.bgp.net.br",
}


@dataclass
class AsInfo:
    export_peer_asns: list[int]
    export_as_any: bool
    import_customer_asns: list[int]
    tech_c: str
    source: str


def deserialize_json(file_path: str):
    with open(file_path, "r") as f:
        data = json.load(f)
    return {int(k): AsInfo(**v) for k, v in data.items()}


def query_whois(tech_c: str, source: str):
    address = SPECIAL_WHOIS_ADDRESS.get(source)
    if address is None:
        address = f"whois.{source.lower()}.net"
    return subprocess.check_output(["whois", "-h", address, tech_c], text=True)


def main():
    relaxed_filter_as_info.download_if_missing()
    as_infos = deserialize_json(relaxed_filter_as_info.path)
    tech_c_map: DefaultDict[str, list[tuple[int, AsInfo]]] = DefaultDict(lambda: list())
    for asn, info in as_infos.items():
        tech_c_map[info.tech_c].append((asn, info))

    if relaxed_filter_tech_c_emails_path not in os.listdir():
        tech_c_outputs: dict[str, str] = {}
        for tech_c, asns in tech_c_map.items():
            output = query_whois(tech_c, asns[0][1].source)
            tech_c_outputs[tech_c] = output

        tech_c_emails: dict[str, str] = {}
        # TODO: Convert whois output to email.
        with open(relaxed_filter_tech_c_emails_path, "a") as f:
            f.write(
                "tech_c,email\n"
                + "".join(
                    f"{tech_c},{email}\n" for tech_c, email in tech_c_emails.items()
                )
            )


main() if __name__ == "__main__" else None

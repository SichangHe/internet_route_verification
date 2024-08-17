"""Run at `scripts/` as `python3 -m scripts.actions.email_mntner`.

Email maintainers of ASes with many `spec_import_customer` or `spec_export_customers`.
Will ask you to input the email address for each `tech-c` if no cache is found.
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


def read_y_n(prompt: str):
    while True:
        match input(f"{prompt} [Y/n]").strip().lower():
            case "", "y", "yes":
                return True
            case "n", "no":
                return False


def input_email():
    while True:
        email = input("What is their email? (Leave blank to skip.)").strip()
        if email == "":
            if read_y_n("Skipping this one email?"):
                return None
        else:
            if read_y_n(f"Your input is `{email}`. Correct?"):
                return email


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
        tech_c_wo_email: set[str] = set()
        for tech_c, output in tech_c_outputs.items():
            print(f"{tech_c} from {tech_c_map[tech_c][0][1].source}:\n{output}")
            if (email := input_email()) is not None:
                tech_c_emails[tech_c] = email
            else:
                tech_c_wo_email.add(tech_c)

        # TODO: Convert whois output to email.
        with open(relaxed_filter_tech_c_emails_path, "a") as f:
            f.write(
                "tech_c,email\n"
                + "".join(
                    f"{tech_c},{email}\n" for tech_c, email in tech_c_emails.items()
                )
            )


main() if __name__ == "__main__" else None

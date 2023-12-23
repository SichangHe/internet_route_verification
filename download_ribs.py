#!/usr/bin/env python3
"""Adapted from Bash scripts in
<https://github.com/cunha/measurements/tree/main/bgp/bgp-downloader>."""
import os
import subprocess


def parallel_download(urls_filenames: list[tuple[str, str]]):
    aria_input = "\n".join(
        [f"{url}\n\tout={filename}" for url, filename in urls_filenames]
    )
    aria_input_file = "aria-input.txt"

    with open(aria_input_file, "w") as f:
        f.write(aria_input)

    aria_command = [
        "aria2c",
        "--continue",
        "--max-connection-per-server=8",
        f"--input-file={aria_input_file}",
        "--log=log.txt",
        "--log-level=info",
    ]

    try:
        subprocess.run(aria_command, check=True)
    except FileNotFoundError:
        print("Please install `aria2c`.")
        exit(1)


years = [2023]
months = [6]
days = [23]
hours = [0]
minutes = [0]

route_view_collector2path = {
    "route-views2": "https://archive.routeviews.org/bgpdata",
    "route-views3": "https://archive.routeviews.org/route-views3/bgpdata",
    "route-views4": "https://archive.routeviews.org/route-views4/bgpdata",
    "route-views5": "https://archive.routeviews.org/route-views5/bgpdata",
    "route-views6": "https://archive.routeviews.org/route-views6/bgpdata",
    "route-views.decix-jhb": "https://archive.routeviews.org/decix.jhb/bgpdata",
    "route-views.amsix": "https://archive.routeviews.org/route-views.amsix/bgpdata",
    "route-views.chicago": "https://archive.routeviews.org/route-views.chicago/bgpdata",
    "route-views.chile": "https://archive.routeviews.org/route-views.chile/bgpdata",
    "route-views.eqix": "https://archive.routeviews.org/route-views.eqix/bgpdata",
    "route-views.flix": "https://archive.routeviews.org/route-views.flix/bgpdata",
    "route-views.gorex": "https://archive.routeviews.org/route-views.gorex/bgpdata",
    "route-views.isc": "https://archive.routeviews.org/route-views.isc/bgpdata",
    "route-views.kixp": "https://archive.routeviews.org/route-views.kixp/bgpdata",
    "route-views.linx": "https://archive.routeviews.org/route-views.linx/bgpdata",
    "route-views.napafrica": "https://archive.routeviews.org/route-views.napafrica/bgpdata",
    "route-views.nwax": "https://archive.routeviews.org/route-views.nwax/bgpdata",
    "pacwave.lax": "https://archive.routeviews.org/pacwave.lax/bgpdata",
    "route-views.phoix": "https://archive.routeviews.org/route-views.phoix/bgpdata",
    "route-views.telxatl": "https://archive.routeviews.org/route-views.telxatl/bgpdata",
    "route-views.wide": "https://archive.routeviews.org/route-views.wide/bgpdata",
    "route-views.sydney": "https://archive.routeviews.org/route-views.sydney/bgpdata",
    "route-views2.saopaulo": "https://archive.routeviews.org/route-views2.saopaulo/bgpdata",
    "route-views.sg": "https://archive.routeviews.org/route-views.sg/bgpdata",
    "route-views.perth": "https://archive.routeviews.org/route-views.perth/bgpdata",
    "route-views.peru": "https://archive.routeviews.org/route-views.peru/bgpdata",
    "route-views.sfmix": "https://archive.routeviews.org/route-views.sfmix/bgpdata",
    "route-views.soxrs": "https://archive.routeviews.org/route-views.soxrs/bgpdata",
    "route-views.mwix": "https://archive.routeviews.org/route-views.mwix/bgpdata",
    "route-views.rio": "https://archive.routeviews.org/route-views.rio/bgpdata",
    "route-views.fortaleza": "https://archive.routeviews.org/route-views.fortaleza/bgpdata",
    "route-views.gixa": "https://archive.routeviews.org/route-views.gixa/bgpdata",
    "route-views.bdix": "https://archive.routeviews.org/route-views.bdix/bgpdata",
    "route-views.bknix": "https://archive.routeviews.org/route-views.bknix/bgpdata",
    "route-views.uaeix": "https://archive.routeviews.org/route-views.uaeix/bgpdata",
    "route-views.ny": "https://archive.routeviews.org/route-views.ny/bgpdata",
    # "route-views.jinx": "https://archive.routeviews.org/route-views.jinx/bgpdata", # Old
    # "pit.scl": "https://archive.routeviews.org/pit.scl/bgpdata", # New after Aug 2023
    # "route-views.saopaulo": "https://archive.routeviews.org/route-views.saopaulo/bgpdata", # Old
    # "route-views.siex": "https://archive.routeviews.org/route-views.siex/bgpdata", # Old
    # "route-views.ipv6": "https://archive.routeviews.org/ipv6", # Old
    # "route-views3-damp": "https://archive.routeviews.org/route-views3-damp", # Old
    # "oix-route-views-damp": "https://archive.routeviews.org/oix-route-views-damp", # Old
}

oix_route_view_collector2path = {
    "oix-route-views": "https://archive.routeviews.org/oix-route-views",
}

ris_collectors = [
    "rrc00",
    "rrc01",
    "rrc03",
    "rrc04",
    "rrc05",
    "rrc06",
    "rrc07",
    "rrc10",
    "rrc11",
    "rrc12",
    "rrc13",
    "rrc14",
    "rrc15",
    "rrc16",
    "rrc18",
    "rrc19",
    "rrc20",
    "rrc21",
    "rrc22",
    "rrc23",
    "rrc24",
    "rrc25",
    "rrc26",
]

DIR = "data/ribs"


def route_view_download_tasks():
    return [
        (
            f"{url_path}/{YYYY}.{mm:02d}/RIBS/rib.{YYYY}{mm:02d}{dd:02d}.{HH:02d}00.bz2",
            f"{DIR}/{collector}-rib.{YYYY}{mm:02d}{dd:02d}.{HH:02d}00.bz2",
        )
        for collector, url_path in route_view_collector2path.items()
        for YYYY in years
        for mm in months
        for dd in days
        for HH in hours
    ] + [
        (
            f"{url_path}/{YYYY}.{mm:02d}/oix-full-snapshot-{YYYY}-{mm:02d}-{dd:02d}-{HH:02d}00.bz2",
            f"{DIR}/{collector}-oix-full-snapshot-{YYYY}-{mm:02d}-{dd:02d}-{HH:02d}00.bz2",
        )
        for collector, url_path in oix_route_view_collector2path.items()
        for YYYY in years
        for mm in months
        for dd in days
        for HH in hours
    ]


def ripe_ris_download_tasks():
    return [
        (
            f"https://data.ris.ripe.net/{collector}/{YYYY}.{mm:02d}/bview.{YYYY}{mm:02d}{dd:02d}.{HH:02d}00.gz",
            f"{DIR}/{collector}-bview.{YYYY}{mm:02d}{dd:02d}.{HH:02d}00.gz",
        )
        for collector in ris_collectors
        for YYYY in years
        for mm in months
        for dd in days
        for HH in hours
    ]


def main():
    os.makedirs(DIR, exist_ok=True)
    url_filenames = route_view_download_tasks() + ripe_ris_download_tasks()
    try:
        parallel_download(url_filenames)
    except KeyboardInterrupt:
        exit(1)


main() if __name__ == "__main__" else None

"""Run at `scripts/` with `python3 -m scripts.download_src_data`.

Downloads all the source data files used in the project to `data/`.
"""

import os

from scripts import CsvFile, download_csv_files_if_missing

as_rel = CsvFile(
    "../data/20230701.as-rel.bz2",
    "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/20230701.as-rel.bz2",
)
"""From <https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data>."""

irrs = CsvFile(
    "../data/irrs.tar.zst",
    "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/irrs.tar.zst",
)
"""From <https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data>."""

single_rib = CsvFile(
    "../data/mrts/rib.20230619.2200.bz2",
    "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rib.20230619.2200.bz2",
)
"""From <https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data>."""

ribs = [
    CsvFile(
        "../data/ribs/oix-route-views--oix-full-snapshot-2023-06-23-0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/oix-route-views--oix-full-snapshot-2023-06-23-0000.bz2",
    ),
    CsvFile(
        "../data/ribs/pacwave.lax--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/pacwave.lax--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.amsix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.amsix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.bdix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.bdix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.bknix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.bknix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.chicago--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.chicago--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.chile--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.chile--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.decix-jhb--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.decix-jhb--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.eqix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.eqix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.flix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.flix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.fortaleza--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.fortaleza--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.gixa--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.gixa--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.gorex--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.gorex--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.isc--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.isc--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.kixp--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.kixp--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.linx--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.linx--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.mwix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.mwix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.napafrica--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.napafrica--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.nwax--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.nwax--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.ny---rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.ny---rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.perth--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.perth--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.peru--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.peru--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.phoix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.phoix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.rio--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.rio--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.sfmix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.sfmix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.sg--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.sg--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.soxrs--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.soxrs--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.sydney--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.sydney--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.telxatl--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.telxatl--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.uaeix--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.uaeix--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views.wide--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views.wide--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views2--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views2--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views2.saopaulo--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views2.saopaulo--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views3--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views3--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views4--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views4--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views5--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views5--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/route-views6--rib.20230623.0000.bz2",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/route-views6--rib.20230623.0000.bz2",
    ),
    CsvFile(
        "../data/ribs/rrc00--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc00--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc01--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc01--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc03--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc03--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc04--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc04--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc05--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc05--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc06--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc06--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc07--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc07--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc10--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc10--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc11--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc11--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc12--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc12--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc13--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc13--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc14--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc14--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc15--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc15--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc16--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc16--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc18--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc18--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc19--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc19--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc20--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc20--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc21--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc21--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc22--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc22--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc23--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc23--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc24--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc24--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc25--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc25--bview.20230623.0000.gz",
    ),
    CsvFile(
        "../data/ribs/rrc26--bview.20230623.0000.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/raw-data/rrc26--bview.20230623.0000.gz",
    ),
]
"""From <https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data>."""


def main():
    download_csv_files_if_missing(ribs + [as_rel, irrs, single_rib])
    assert os.system(f"tar --zstd -xf {irrs.path} -C ../data/") == 0


main() if __name__ == "__main__" else None

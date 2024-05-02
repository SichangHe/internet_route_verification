import gc

from scripts.fig import (
    as_all_stacked_area_distin,
    as_pair_all_stacked_area,
    as_rules_cdf,
    as_spec_all_stacked_area,
    as_unrec_all_stacked_area,
    route_all_stacked_area,
)


def main():
    mods = [
        as_all_stacked_area_distin,
        as_pair_all_stacked_area,
        as_spec_all_stacked_area,
        as_rules_cdf,
        route_all_stacked_area,
        as_unrec_all_stacked_area,
    ]
    for mod in mods:
        print(f"Running {mod.__name__}.")
        mod.main()
        gc.collect()


if __name__ == "__main__":
    main()

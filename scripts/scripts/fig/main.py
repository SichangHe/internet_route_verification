import gc

from scripts.fig import (
    as_pair_stacked_area,
    as_rules_cdf,
    as_spec_stacked_area,
    as_stacked_area,
    as_unrec_stacked_area,
    route_port_stacked_area,
)


def main():
    mods = [
        as_pair_stacked_area,
        as_rules_cdf,
        as_spec_stacked_area,
        as_stacked_area,
        as_unrec_stacked_area,
        route_port_stacked_area,
    ]
    for mod in mods:
        print(f"Running {mod.__name__}.")
        mod.main()
        gc.collect()


if __name__ == "__main__":
    main()

from typing import Iterable


def merge_afi_dict(afis: Iterable[dict[str, str]]) -> set[tuple[str, str]]:
    return merge_afi(
        (afi_item["version"], afi_item.get("cast", "any")) for afi_item in afis
    )


def merge_afi(afis: Iterable[tuple[str, str]]) -> set[tuple[str, str]]:
    afi_sets: dict[str, set[str]] = {}
    for version, cast in afis:
        entry = afi_sets.get(version, set())
        entry.add(cast)
        afi_sets[version] = entry
    afi_map: dict[str, str] = {}
    for key, afi_set in afi_sets.items():
        if "any" in afi_set or ("unicast" in afi_set and "multicast" in afi_set):
            afi_map[key] = "any"
        else:
            assert len(afi_set) == 1
            afi_map[key] = afi_set.pop()
    if (v4 := afi_map.get("ipv4")) and (v6 := afi_map.get("ipv6")) and (v4 == v6):
        return set([("any", v4)])
    return set((key, value) for key, value in afi_map.items())


def unwrap_afi_set_version(afis: set[tuple[str, str]]):
    new = afis.copy()
    for version, cast in afis:
        if version == "any":
            for version in ("ipv4", "ipv6"):
                new.add((version, cast))
        new.remove(("any", cast))
    return new


def unwrap_afi_set_cast(afis: set[tuple[str, str]]):
    new = afis.copy()
    for version, cast in afis:
        if cast == "any":
            for cast in ("unicast", "multicast"):
                new.add((version, cast))
        new.remove((version, "any"))
    return new


def unwrap_afi_set(afis: set[tuple[str, str]]):
    afis = unwrap_afi_set_version(afis)
    afis = unwrap_afi_set_cast(afis)
    return afis


def afi_set_intersection_difference(
    left: set[tuple[str, str]], right: set[tuple[str, str]]
):
    left_unwrap = unwrap_afi_set(left)
    right_unwrap = unwrap_afi_set(right)
    intersection = left_unwrap.intersection(right_unwrap)
    difference = left_unwrap.difference(right_unwrap)
    return merge_afi(intersection), merge_afi(difference)

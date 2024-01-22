from dataclasses import dataclass


@dataclass
class RPSLObject:
    closs: str
    name: str
    body: str


@dataclass
class AutNum:
    """aut-num class."""

    name: str
    body: str
    n_import: int
    n_export: int
    imports: dict[str, dict[str, list[dict]]]
    exports: dict[str, dict[str, list[dict]]]


@dataclass
class AsSet:
    """as-set class."""

    name: str
    body: str
    members: list[str]


@dataclass
class RouteSet:
    """route-set class."""

    name: str
    body: str
    members: list[str]
    """members and mp-members."""


@dataclass
class PeeringSet:
    """peering-set class."""

    name: str
    body: str
    peerings: list[dict]


@dataclass
class FilterSet:
    """filter-set class."""

    name: str
    body: str
    filters: list[dict]

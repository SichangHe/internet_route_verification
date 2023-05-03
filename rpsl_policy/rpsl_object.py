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
    imports: list[dict]
    exports: list[dict]


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

"""
Parse mp-import statement following
https://www.rfc-editor.org/rfc/rfc4012#section-2.5
"""
from pyparsing import (
    CaselessKeyword,
    Forward,
    Group,
    OneOrMore,
    Opt,
    Word,
    ZeroOrMore,
    delimited_list,
    printables,
)

exclude_chars = "#;"
field = Word(printables, exclude_chars=exclude_chars)
field_wo_comma = Word(printables, exclude_chars=(exclude_chars + ","))
field_w_space = Word(printables + " ", exclude_chars=exclude_chars)
semicolon = Word(";").suppress()
from_kw = CaselessKeyword("from")
action_kw = CaselessKeyword("action")
accept_kw = CaselessKeyword("accept")
and_kw = CaselessKeyword("and")
or_kw = CaselessKeyword("or")
except_kw = CaselessKeyword("except")
protocol = CaselessKeyword("protocol") + field("protocol-1")
into_protocol = "into" + field("protocol-2")
afi = CaselessKeyword("afi") + delimited_list(
    field_wo_comma, delim=","
).set_results_name("afi-list")
action = field_w_space + semicolon
actions = action_kw + Group(
    ZeroOrMore(action + ~(from_kw | accept_kw)) + action
).set_results_name("actions")
mp_peering_raw = Group(ZeroOrMore(field + ~(action_kw | from_kw | accept_kw)) + field)
peering = Group(from_kw + mp_peering_raw.set_results_name("mp-peering") + Opt(actions))
import_factor = (
    Group(OneOrMore(peering)).set_results_name("from")
    + accept_kw
    + field_w_space("mp-filter")
)
import_term = (
    # Semicolon separated list.
    "{"
    + delimited_list(
        Group(import_factor), delim=";", allow_trailing_delim=True
    ).set_results_name("import-factors")
    + "}"
    # Semicolon optional if single.
) | import_factor + Opt(semicolon)

# `import_expression` and `afi_import_expression` are recursively defined.
import_expression = Forward()
afi_import_expression = Opt(afi) + import_expression
import_expression <<= (
    Group(import_term + CaselessKeyword("except") + afi_import_expression)
    | Group(import_term + CaselessKeyword("refine") + afi_import_expression)
    | import_term
)

lex = Opt(protocol) + Opt(into_protocol) + afi_import_expression

# TODO: parse <mp-filter>.
# TODO: parse <mp-peering>.
operator_except = and_kw | or_kw | except_kw
as_expression = Group(field + ZeroOrMore(operator_except + field))
# Works under my assumption that inet-rtr names and rtr-set names match `field`.
mp_router_expression = as_expression
mp_peering = Group(
    as_expression("as-expression")
    + Opt(mp_router_expression("mp-router-expression-1"))
    + Opt("at" + mp_router_expression("mp-router-expression-2"))
) | field("peering-set-name")

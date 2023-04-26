"""
Parse mp-import statement following
<https://www.rfc-editor.org/rfc/rfc4012#section-2.5>
"""
from pyparsing import (
    CaselessKeyword,
    Forward,
    Group,
    OneOrMore,
    Opt,
    Suppress,
    Word,
    ZeroOrMore,
    alphanums,
    delimited_list,
    printables,
)

exclude_chars = "#;"
simple_field = Word(alphanums)
"""Any alphanumeric characters"""
field = Word(printables, exclude_chars=exclude_chars)
"""Any printable characters except ` `, `#`, `;`"""
field_wo_comma = Word(printables, exclude_chars=(exclude_chars + ","))
"""Any printable characters except ` `, `#`, `;`, `,`"""
field_w_space = Word(printables + " ", exclude_chars=exclude_chars)
"""Any printable characters except `#`, `;`, `,`"""
field_wo_brace = Word(printables, exclude_chars=(exclude_chars + ",(){}"))
"""A field without `,`, `(`, `)`, `{`, `}`"""
field_wo_eq = Word(printables, exclude_chars=(exclude_chars + "="))
"""A field without `=`"""
semicolon = Word(";").suppress()
"""Semicolon, suppressed"""
from_kw = CaselessKeyword("from")
to_kw = CaselessKeyword("to")
action_kw = CaselessKeyword("action")
accept_kw = CaselessKeyword("accept")
announce_kw = CaselessKeyword("announce")
and_kw = CaselessKeyword("and")
or_kw = CaselessKeyword("or")
not_kw = CaselessKeyword("not")
except_kw = CaselessKeyword("except")
refine_kw = CaselessKeyword("refine")
at_kw = CaselessKeyword("at")
community_kw = CaselessKeyword("community")

# -----------------------------------------------------------------------------
# <mp-peering>, not further parsed.
# -----------------------------------------------------------------------------
action_raw = field_w_space + semicolon
"""<action-N>;"""
follows_action = from_kw | to_kw | accept_kw | announce_kw
action_raws = action_kw + Group(
    OneOrMore(~follows_action + action_raw)
).set_results_name("actions")
"""action <action-1>; ... <action-N>;
-> list[str]"""
mp_peering_raw = Group(OneOrMore(~(action_kw | follows_action) + field))
"""<mp-peering-M>
-> list[str]"""
mp_peering_raws = Group(
    (from_kw | to_kw) + mp_peering_raw.set_results_name("mp-peering") + Opt(action_raws)
)
"""from <mp-peering-M> [action <action-1>; ... <action-N>;]
or
to <mp-peering-M> [action <action-1>; ... <action-N>;]
-> {mp-peering: list[str], [actions]: list[str]}"""

# -----------------------------------------------------------------------------
# Structured <mp-import>
# -----------------------------------------------------------------------------
afi = CaselessKeyword("afi") + delimited_list(
    field_wo_comma, delim=","
).set_results_name("afi-list")
"""afi <afi-list>
-> afi-list: list[str]"""
protocol = CaselessKeyword("protocol") + field("protocol-1")
"""protocol <protocol-1>
-> protocol-1: str"""
into_protocol = CaselessKeyword("into") + field("protocol-2")
"""into <protocol-2>
-> protocol-2: str"""
import_factor = (
    Group(OneOrMore(mp_peering_raws)).set_results_name("mp-peerings")
    + (accept_kw | announce_kw)
    + field_w_space("mp-filter")
)
"""<import-factor> ::=
from <mp-peering-1> [action <action-1>; ... <action-M>;]
. . .
from <mp-peering-N> [action <action-1>; ... <action-K>;]
accept <mp-filter>
-> mp-peerings: list[{mp-peering, [actions]}], mp-filter: str
Note: no trailing `;`, different from spec in RFC."""
import_term = (
    # Semicolon separated list.
    "{"
    + delimited_list(
        Group(import_factor), delim=";", allow_trailing_delim=True
    ).set_results_name("import-factors")
    + "}"
    # Semicolon optional if single.
) | import_factor + Opt(semicolon)
""" <import-term> :: = {
<import-factor-1>;
 . . .
<import-factor-N>[;]
} | import-factor[;]
-> import-factors: list[{mp-peerings, mp-filter}] | (mp-peerings, mp-filter)"""

# `import_expression` and `afi_import_expression` are recursively defined.
import_expression = Forward()
"""<import-expression> ::=
<import-term> EXCEPT <afi-import-expression> |
<import-term> REFINE <afi-import-expression> |
<import-term>
-> import-expression: {import-term, logic, [afi-list],...}
| import-factors: list[{mp-peerings, mp-filter}] | (mp-peerings, mp-filter)
"""
afi_import_expression = Opt(afi) + import_expression
"""<afi-import-expression> ::= [afi <afi-list>] <import-expression>
[afi-list]: list[str], (
    import-expression: {import-term, logic, [afi-list], ...}
    | import-factors: list[{mp-peerings, mp-filter}]
    | (mp-peerings, mp-filter)
)"""
import_expression <<= (
    # TODO: Verify that `EXCEPT` and `REFINE` work.
    Group(import_term("import-term") + except_kw("logic") + afi_import_expression)(
        "import-expression"
    )
    | Group(import_term("import-term") + refine_kw("logic") + afi_import_expression)(
        "import-expression"
    )
    | import_term
)

mp_import = Opt(protocol) + Opt(into_protocol) + afi_import_expression
"""mp-import: [protocol <protocol-1>] [into <protocol-2>]
<afi-import-expression>
<https://www.rfc-editor.org/rfc/rfc4012#section-2.5>
Input should be in one line, without comments. Can also parse `mp-export`.
<action>, <mp-filter>, <mp-peering> in parse results not further parsed.
-> [protocol-1]: str, [protocol-2]: str, [afi-list]: list[str], (
    import-expression: {import-term, logic, [afi-list], ...}
    | import-factors: list[{mp-peerings, mp-filter}]
    | (
        mp-peerings: list[{mp-peering: list[str], [actions]: list[str]}],
        mp-filter: str
    )
)
"""

# -----------------------------------------------------------------------------
# Further parse <mp-peering>
# -----------------------------------------------------------------------------
field_not_at = ~at_kw + field
"""Field that is not `at`"""
fields_not_at_by_and_or_except = Group(
    field_not_at + ZeroOrMore((and_kw | or_kw | except_kw) + field_not_at)
)
"""List of fields that are not `at`, chained by `and`, `or`, or `except`
-> list["and" | "or" | "except" | str]"""
as_expression = fields_not_at_by_and_or_except
"""<as-expression> is an expression over AS numbers and AS sets
using operators AND, OR, and EXCEPT
-> list["and" | "or" | "except" | str]"""
# TODO: Varify that inet-rtr names and rtr-set names match `field`.
mp_router_expression = fields_not_at_by_and_or_except
"""<mp-router-expression> is an expression over router ipv4-addresses or
ipv6-addresses, inet-rtr names, and rtr-set names using operators AND, OR, and
EXCEPT
-> list["and" | "or" | "except" | str]"""
mp_peering = (
    as_expression("as-expression")
    + Opt(mp_router_expression("mp-router-expression-1"))
    + Opt(at_kw + mp_router_expression("mp-router-expression-2"))
) | field("peering-set-name")
"""<mp-peering> ::= <as-expression> [<mp-router-expression-1>]
[at <mp-router-expression-2>] | <peering-set-name>
<https://www.rfc-editor.org/rfc/rfc4012#section-2.5.1>
-> (
    as-expression: list["and" | "or" | "except" | str],
    [mp-router-expression-1]: list["and" | "or" | "except" | str],
    [mp-router-expression-2]: list["and" | "or" | "except" | str]
) | peering-set-name: str"""

# -----------------------------------------------------------------------------
# Further parse community(), community.append(), etc.
# -----------------------------------------------------------------------------
address_prefix_set = Group(
    Suppress("{") + Opt(delimited_list(field_wo_brace, delim=",")) + Suppress("}")
)
"""An explicit list of address prefixes enclosed in braces '{' and '}'
-> list[str]"""
community_field = Group(
    Suppress(community_kw)
    + Opt(Suppress(".") + field_wo_brace("method"))
    + Suppress("(")
    + delimited_list(field_wo_brace, delim=",")("args")
    + Suppress(")")
)
"""community(<arg-1>, ..., <arg-N>)
or
community.method(<arg-1>, ..., <arg-N>)
-> {[method]: str, args: list[str]}"""
community_dot_eq = (
    Suppress(community_kw) + Suppress(".=") + address_prefix_set("add-community")
)
"""community .= {...}
-> add-community: list[str]"""

# -----------------------------------------------------------------------------
# Further parse <mp-filter>
# -----------------------------------------------------------------------------
policy_filter = OneOrMore(~(and_kw | or_kw | not_kw) + Group(
    field_wo_brace("path-attribute") | address_prefix_set("address-prefix-set")
))
"""A logical expression which when applied to a set of routes returns a subset
of these routes
-> list[{path-attribute: str | address-prefix-set: list[str]}]
<https://www.rfc-editor.org/rfc/rfc2622#section-5.4>"""
# `mp_filter` and `mp_filter_base` are recursively defined.
mp_filter = Forward()
"""Policy filter composite by using the operators AND, OR, and NOT
-> and: {left: {...}, right: {...}}
| or: {left: {...}, right: {...}}
| not: {...}
| community: {[method]: str, args: list[str]}
| mp-filter: {...}
| policy-filter: list[{path-attribute: str | address-prefix-set: list[str]}]
<https://www.rfc-editor.org/rfc/rfc4012#section-2.5.2>"""
mp_filter_base = (
    community_field("community")
    | Group(Suppress("(") + mp_filter + Suppress(")"))("mp-filter")
    | policy_filter("policy-filter")
)
"""-> community: {[method]: str, args: list[str]}
| mp-filter: {...}
| policy-filter: list[{path-attribute: str | address-prefix-set: list[str]}]"""
mp_filter_not = Group(Suppress(not_kw) + mp_filter)("not")
mp_filter_and = Group(
    Group(mp_filter_base)("left") + Suppress(and_kw) + Group(mp_filter)("right")
)("and")
mp_filter_or = Group(
    Group(mp_filter_base)("left") + Suppress(or_kw) + Group(mp_filter)("right")
)("or")
mp_filter <<= mp_filter_and | mp_filter_or | mp_filter_not | mp_filter_base

# -----------------------------------------------------------------------------
# Further parse <action>
# -----------------------------------------------------------------------------
assignment = Group(
    field_wo_eq("assignee") + "=" + (field_wo_eq | address_prefix_set)("assigned")
)
"""<assignee>=<assigned>
or
<assignee>={ addr-1, ... }"""
method_call = Group(
    simple_field("rp-attribute")
    + Suppress(".")
    + field_wo_brace("method")
    + Suppress("(")
    + delimited_list(field_wo_brace, delim=",")("args")
    + Suppress(")")
)
"""rp-attribute.method(<arg-1>, ..., <arg-N>)"""
action = (
    assignment("assignment")
    | community_field("community")
    | community_dot_eq
    | method_call("method-call")
)
"""assignee = assigned, community(), community.method(), community .= <assigned>
or rp-attribute.method()

<https://www.rfc-editor.org/rfc/rfc2622#page-43>"""

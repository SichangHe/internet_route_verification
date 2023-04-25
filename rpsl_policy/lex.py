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
    delimited_list,
    printables,
)

exclude_chars = "#;"
field = Word(printables, exclude_chars=exclude_chars)
"""Any printable characters except ` `, `#`, `;`"""
field_wo_comma = Word(printables, exclude_chars=(exclude_chars + ","))
"""Any printable characters except ` `, `#`, `;`, `,`"""
field_w_space = Word(printables + " ", exclude_chars=exclude_chars)
"""Any printable characters except `#`, `;`, `,`"""
field_wo_brace = Word(printables, exclude_chars=(exclude_chars + ",(){}"))
"""A field without `,`, `(`, `)`, `{`, `}`"""
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
protocol = CaselessKeyword("protocol") + field("protocol-1")
"""protocol <protocol-1>"""
into_protocol = CaselessKeyword("into") + field("protocol-2")
"""into <protocol-2>"""
afi = CaselessKeyword("afi") + delimited_list(
    field_wo_comma, delim=","
).set_results_name("afi-list")
"""afi <afi-list>"""

# -----------------------------------------------------------------------------
# <mp-peering>, not further parsed.
# -----------------------------------------------------------------------------
action_raw = field_w_space + semicolon
"""<action-N>;"""
follows_action = from_kw | to_kw | accept_kw | announce_kw
action_raws = action_kw + Group(
    OneOrMore(~follows_action + action_raw)
).set_results_name("actions")
"""action <action-1>; ... <action-N>;"""
mp_peering_raw = Group(OneOrMore(~(action_kw | follows_action) + field))
"""<mp-peering-M>"""
mp_peering_raws = Group(
    (from_kw | to_kw) + mp_peering_raw.set_results_name("mp-peering") + Opt(action_raws)
)
"""from <mp-peering-M> [action <action-1>; ... <action-N>;]
or
to <mp-peering-M> [action <action-1>; ... <action-N>;]"""

# -----------------------------------------------------------------------------
# Structured <mp-import>
# Remember to change below <mp-export> code if changing this.
# -----------------------------------------------------------------------------
import_factor = (
    Group(OneOrMore(mp_peering_raws)).set_results_name("from")
    + accept_kw
    + field_w_space("mp-filter")
)
"""<import-factor> ::=
from <mp-peering-1> [action <action-1>; ... <action-M>;]
. . .
from <mp-peering-N> [action <action-1>; ... <action-K>;]
accept <mp-filter>

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
} | import-factor[;]"""

# `import_expression` and `afi_import_expression` are recursively defined.
import_expression = Forward()
"""<import-expression> ::=
<import-term> EXCEPT <afi-import-expression> |
<import-term> REFINE <afi-import-expression> |
<import-term>
"""
afi_import_expression = Opt(afi) + import_expression
"""<afi-import-expression> ::= [afi <afi-list>] <import-expression>"""
import_expression <<= (
    Group(import_term + except_kw + afi_import_expression)
    | Group(import_term + refine_kw + afi_import_expression)
    | import_term
)

mp_import = Opt(protocol) + Opt(into_protocol) + afi_import_expression
"""mp-import: [protocol <protocol-1>] [into <protocol-2>]
<afi-import-expression>

<https://www.rfc-editor.org/rfc/rfc4012#section-2.5>

Input should be in one line, without comments.
<action>, <mp-filter>, <mp-peering> in parse results not further parsed.
"""

# -----------------------------------------------------------------------------
# Structured <mp-export>
# Remember to change above <mp-import> code if changing this.
# -----------------------------------------------------------------------------
export_factor = (
    Group(OneOrMore(mp_peering_raws)).set_results_name("to")
    + announce_kw
    + field_w_space("mp-filter")
)
"""<export-factor> ::=
to <mp-peering-1> [action <action-1>; ... <action-M>;]
. . .
to <mp-peering-N> [action <action-1>; ... <action-K>;]
announce <mp-filter>

Note: no trailing `;`, different from spec in RFC."""
export_term = (
    # Semicolon separated list.
    "{"
    + delimited_list(
        Group(export_factor), delim=";", allow_trailing_delim=True
    ).set_results_name("export-factors")
    + "}"
    # Semicolon optional if single.
) | export_factor + Opt(semicolon)
""" <export-term> :: = {
<export-factor-1>;
 . . .
<export-factor-N>[;]
} | export-factor[;]"""

# `export_expression` and `afi_export_expression` are recursively defined.
export_expression = Forward()
"""<export-expression> ::=
<export-term> EXCEPT <afi-export-expression> |
<export-term> REFINE <afi-export-expression> |
<export-term>
"""
afi_export_expression = Opt(afi) + export_expression
"""<afi-export-expression> ::= [afi <afi-list>] <export-expression>"""
export_expression <<= (
    Group(export_term + except_kw + afi_export_expression)
    | Group(export_term + refine_kw + afi_export_expression)
    | export_term
)

mp_export = Opt(protocol) + Opt(into_protocol) + afi_export_expression
"""mp-export: [protocol <protocol-1>] [into <protocol-2>]
<afi-export-expression>

<https://www.rfc-editor.org/rfc/rfc4012#section-2.5>

Input should be in one line, without comments.
<action>, <mp-filter>, <mp-peering> in parse results not further parsed.
"""

# -----------------------------------------------------------------------------
# Further parse <mp-peering>
# -----------------------------------------------------------------------------
field_not_at = ~at_kw + field
"""Field that is not `at`"""
fields_not_at_by_and_or_except = Group(
    field_not_at + ZeroOrMore((and_kw | or_kw | except_kw) + field_not_at)
)
"""List of fields that are not `at`, chained by `and`, `or`, or `except`"""
as_expression = fields_not_at_by_and_or_except
"""<as-expression> is an expression over AS numbers and AS sets
using operators AND, OR, and EXCEPT"""
# TODO: Varify that inet-rtr names and rtr-set names match `field`.
mp_router_expression = fields_not_at_by_and_or_except
"""<mp-router-expression> is an expression over router ipv4-addresses or
ipv6-addresses, inet-rtr names, and rtr-set names using operators AND, OR, and
EXCEPT"""
mp_peering = (
    as_expression("as-expression")
    + Opt(mp_router_expression("mp-router-expression-1"))
    + Opt(at_kw + mp_router_expression("mp-router-expression-2"))
) | field("peering-set-name")
"""<mp-peering> ::= <as-expression> [<mp-router-expression-1>]
[at <mp-router-expression-2>] | <peering-set-name>

<https://www.rfc-editor.org/rfc/rfc4012#section-2.5.1>"""

# -----------------------------------------------------------------------------
# Further parse <mp-filter>
# -----------------------------------------------------------------------------
community_field = Group(
    Suppress(community_kw)
    + Opt(Suppress(".") + field_wo_brace("method"))
    + Suppress("(")
    + delimited_list(field_wo_brace, delim=",")("args")
    + Suppress(")")
)
"""community(<arg-1>, ..., <arg-N>)
or
community.method(<arg-1>, ..., <arg-N>)"""
path_attribute = community_field("community") | Group(
    OneOrMore(~(and_kw | or_kw | not_kw) + field_wo_brace)
).set_results_name("path-attribute")
"""Path attribute
<https://www.rfc-editor.org/rfc/rfc4271.html#section-5>"""
address_prefix_set = Group(
    Suppress("{") + Opt(delimited_list(field_wo_brace, delim=",")) + Suppress("}")
)
"""An explicit list of address prefixes enclosed in braces '{' and '}'"""
policy_filter = address_prefix_set("address-prefix-set") | path_attribute
"""A logical expression which when applied to a set of routes returns a subset
of these routes
<https://www.rfc-editor.org/rfc/rfc2622#section-5.4>"""
mp_filter = Forward()
"""Policy filter composite by using the operators AND, OR, and NOT
<https://www.rfc-editor.org/rfc/rfc4012#section-2.5.2>"""
mp_filter_item = Opt(not_kw("modifier")) + (
    (Suppress("(") + mp_filter + Suppress(")")) | policy_filter
)
mp_filter <<= Group(
    mp_filter_item
    + Opt(OneOrMore(Group((and_kw | or_kw)("logic") + mp_filter_item))("nested"))
).set_results_name("mp-filter")

# TODO: Parse <action>: https://www.rfc-editor.org/rfc/rfc2622#page-43

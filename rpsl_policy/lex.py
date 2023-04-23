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
semicolon = Word(";").suppress()
"""Semicolon, suppressed"""
from_kw = CaselessKeyword("from")
action_kw = CaselessKeyword("action")
accept_kw = CaselessKeyword("accept")
and_kw = CaselessKeyword("and")
or_kw = CaselessKeyword("or")
except_kw = CaselessKeyword("except")
at_kw = CaselessKeyword("at")
protocol = CaselessKeyword("protocol") + field("protocol-1")
"""protocol <protocol-1>"""
into_protocol = CaselessKeyword("into") + field("protocol-2")
"""into <protocol-2>"""
afi = CaselessKeyword("afi") + delimited_list(
    field_wo_comma, delim=","
).set_results_name("afi-list")
"""afi <afi-list>"""
action_raw = field_w_space + semicolon
"""<action-N>;"""
action_raws = action_kw + Group(
    OneOrMore(~(from_kw | accept_kw) + action_raw)
).set_results_name("actions")
"""action <action-1>; ... <action-N>;"""
mp_peering_raw = Group(OneOrMore(~(action_kw | from_kw | accept_kw) + field))
"""<mp-peering-M>"""
mp_peering_raws = Group(from_kw + mp_peering_raw.set_results_name("mp-peering") + Opt(action_raws))
"""from <mp-peering-M> [action <action-1>; ... <action-N>;]"""
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
    | Group(import_term + CaselessKeyword("refine") + afi_import_expression)
    | import_term
)

mp_import = Opt(protocol) + Opt(into_protocol) + afi_import_expression
"""mp-import: [protocol <protocol-1>] [into <protocol-2>]
<afi-import-expression>

<https://www.rfc-editor.org/rfc/rfc4012#section-2.5>

Input should be in one line, without comments.
<action>, <mp-filter>, <mp-peering> in parse results not further parsed.
"""

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
# TODO: parse <mp-filter>: https://www.rfc-editor.org/rfc/rfc4012#section-2.5.2
# TODO: Parse <action>: https://www.rfc-editor.org/rfc/rfc2622#page-43

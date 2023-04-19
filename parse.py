from pyparsing import (
    Group,
    Keyword,
    OneOrMore,
    Opt,
    Suppress,
    Word,
    alphanums,
    delimited_list,
)

EXAMPLES = [
    "afi ipv6.unicast from AS9002 accept ANY",
    "afi ipv6.unicast from AS9002 from AS2356 accept ANY",
    "afi ipv6.unicast from AS6939 action pref=100; accept ANY",
    "afi ipv6.unicast from AS21127 action pref=100; accept AS-ZSTTK6-SET;",
    "afi ipv6.unicast from AS21127 action pref=100; med=0; accept AS-ZSTTK6-SET;",
    "afi ipv6 from AS1213 accept { ::/0 }",
    "afi ipv6.unicast from AS1299 action pref=200; accept ANY AND NOT {0.0.0.0/0};",
    # "afi ipv6.unicast from AS1299 action pref = 200; accept ANY AND NOT {0.0.0.0/0};",
    # "afi ipv4.unicast from AS6682 at 109.68.121.1 action pref=65435; med=0; community.append(8226:1102); accept ANY AND {0.0.0.0/0^0-24}",
]

field = Word(alphanums + "^-+=:./")
semicolon = Word(";").suppress()
protocol = "protocol" + field("protocol-1")
into_protocol = "into" + field("protocol-2")
afi = "afi" + field("afi-list")
actions = "action" + Group(OneOrMore(field + semicolon)).set_results_name("actions")
peering = Group(
    "from"
    # TODO: deal with <mp-peering>.
    + field("mp-peering")
    + Opt(actions)
)
address_filter = Suppress("{") + Group(delimited_list(field)) + Suppress("}")
filter_set = "fltr" + field
filter_operator = Keyword("AND") | Keyword("OR")
# TODO: preserve delimiters.
filter = delimited_list(
    Group(Opt(Keyword("NOT")) + (address_filter | filter_set | field)),
    delim=filter_operator,
)
lex = (
    Opt(protocol)
    + Opt(into_protocol)
    + Opt(afi)
    + Group(OneOrMore(peering)).set_results_name("from")
    + "accept"
    + Group(filter).set_results_name("mp-filter")
    + Opt(semicolon)
)

for example in EXAMPLES:
    print(f"\n{example} ->")
    results = lex.parseString(example)
    print(results.as_dict())

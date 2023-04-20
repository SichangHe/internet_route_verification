from pyparsing import Group, Keyword, OneOrMore, Opt, Word, ZeroOrMore, printables

EXAMPLES = [
    "afi ipv6.unicast from AS9002 accept ANY",
    "afi ipv6.unicast from AS9002 from AS2356 accept ANY",
    "afi ipv6.unicast from AS6939 action pref=100; accept ANY",
    "afi ipv6.unicast from AS21127 action pref=100; accept AS-ZSTTK6-SET;",
    "afi ipv6.unicast from AS21127 action pref=100; med=0; accept AS-ZSTTK6-SET;",
    "afi ipv6 from AS1213 accept { ::/0 }",
    "afi ipv6.unicast from AS1299 action pref = 200; accept ANY AND NOT {0.0.0.0/0};",
    "afi ipv4.unicast from AS6682 at 109.68.121.1 action pref=65435; med=0; community.append(8226:1102); accept ANY AND {0.0.0.0/0^0-24}",
]

exclude_chars = "#;"
field = Word(printables, exclude_chars=exclude_chars)
field_w_space = Word(printables + " ", exclude_chars=exclude_chars)
semicolon = Word(";").suppress()
protocol = "protocol" + field("protocol-1")
into_protocol = "into" + field("protocol-2")
afi = "afi" + field("afi-list")
action = field + Opt("=" + field) + semicolon
actions = "action" + Group(OneOrMore(action)).set_results_name("actions")
mp_peering = Group(
    ZeroOrMore(field + ~(Keyword("action") | Keyword("from") | Keyword("accept")))
    + field
)
peering = Group("from" + mp_peering.set_results_name("mp-peering") + Opt(actions))
comment = "#" + field_w_space("comment")
lex = (
    Opt(protocol)
    + Opt(into_protocol)
    + Opt(afi)
    + Group(OneOrMore(peering)).set_results_name("from")
    + "accept"
    + field_w_space("mp-filter")
    + Opt(semicolon)
    + Opt(comment)
)

# TODO: parse <mp-filter>.
# TODO: parse <mp-peering>.


def main():
    for example in EXAMPLES:
        success, results = lex.run_tests(example, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore


if __name__ == "__main__":
    main()

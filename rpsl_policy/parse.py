from pyparsing import Group, Keyword, OneOrMore, Opt, Word, ZeroOrMore, printables

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
    EXAMPLES = [
        "afi ipv6.unicast from AS9002 accept ANY",
        "afi ipv6.unicast from AS9002 from AS2356 accept ANY",
        "afi ipv6.unicast from AS6939 action pref=100; accept ANY",
        "afi ipv6.unicast from AS21127 action pref=100; accept AS-ZSTTK6-SET;",
        "afi ipv6.unicast from AS21127 action pref=100; med=0; accept AS-ZSTTK6-SET;",
        "afi ipv6 from AS1213 accept { ::/0 }",
        "afi ipv6.unicast from AS1299 action pref = 200; accept ANY AND NOT {0.0.0.0/0};",
        "afi ipv4.unicast from AS6682 at 109.68.121.1 action pref=65435; med=0; community.append(8226:1102); accept ANY AND {0.0.0.0/0^0-24}",
        "afi ipv6.unicast from AS8717 2001:67c:20d0:fffe:ffff:ffff:ffff:fffe at 2001:67c:20d0:fffe:ffff:ffff:ffff:fffd accept ANY; # SPECTRUMNET",
        # TODO: Handle following statement with braces.
        "afi ipv4.unicast { from AS174 192.38.7.14 at 192.38.7.75 accept AS174; from AS1835 192.38.7.1 at 192.38.7.75 accept AS-UNIC; from AS2603 192.38.7.50 at 192.38.7.75 accept AS-NORDUNET; from AS2686 192.38.7.8 at 192.38.7.75 accept AS-IGNEMEA; from AS2874 192.38.7.10 at 192.38.7.75 accept AS-GLOBALIPNET; from AS6834 192.38.7.4 at 192.38.7.75 accept AS-KMD; from AS8434 192.38.7.92 at 192.38.7.75 accept AS-TELENOR; from AS9120 192.38.7.46 at 192.38.7.75 accept AS-COHAESIO; from AS9167 192.38.7.49 at 192.38.7.75 accept AS-WEBPARTNER; from AS12552 192.38.7.68 at 192.38.7.75 accept AS-IPO; from AS13030 192.38.7.52 at 192.38.7.75 accept AS-INIT7; from AS15516 192.38.7.64 at 192.38.7.75 accept AS-DK-ARROWHEAD; from AS15598 192.38.7.84 at 192.38.7.75 accept AS-IPX; from AS16095 192.38.7.66 at 192.38.7.75 accept AS-JAYNET; from AS16095 192.38.7.67 at 192.38.7.75 accept AS-JAYNET; from AS16150 192.38.7.43 at 192.38.7.75 accept AS16150:AS-CUSTOMERS; from AS16245 192.38.7.93 at 192.38.7.75 accept AS-NGDC; from AS20618 192.38.7.99 at 192.38.7.75 accept AS-INFOCONNECT; from AS20618 192.38.7.100 at 192.38.7.75 accept AS-INFOCONNECT; from AS25384 192.38.7.97 at 192.38.7.75 accept AS-DMDATADK; from AS25384 192.38.7.98 at 192.38.7.75 accept AS-DMDATADK; from AS28717 192.38.7.82 at 192.38.7.75 accept AS-ZENSYSTEMS; from AS29100 192.38.7.77 at 192.38.7.75 accept AS29100; from AS29266 192.38.7.41 at 192.38.7.75 accept AS-DANMARKSRADIO; from AS31027 192.38.7.58 at 192.38.7.75 accept AS-NIANET; from AS31661 192.38.7.12 at 192.38.7.75 accept AS-COMX; from AS33916 192.38.7.87 at 192.38.7.75 accept AS33916; from AS33926 192.38.7.72 at 192.38.7.75 accept AS-EUROTRANSIT; from AS34823 192.38.7.95 at 192.38.7.75 accept AS34823; from AS41025 192.38.7.28 at 192.38.7.75 accept AS-BUTLERNET; from AS42525 192.38.7.26 at 192.38.7.75 accept AS-GCNET; from AS43457 192.38.7.73 at 192.38.7.75 accept AS-VGDC; }",
    ]

    for example in EXAMPLES:
        success, results = lex.run_tests(example, full_dump=False)
        if success:
            print(results[0][1].as_dict())  # type: ignore


if __name__ == "__main__":
    main()

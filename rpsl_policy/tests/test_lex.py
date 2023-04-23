from pyparsing import ParseResults

from ..lex import mp_import, mp_peering

MP_IMPORT_EXAMPLES = [
    "afi ipv6.unicast from AS9002 accept ANY",
    "afi ipv6.unicast from AS9002 from AS2356 accept ANY",
    "afi ipv6.unicast from AS6939 action pref=100; accept ANY",
    "afi ipv6.unicast from AS21127 action pref=100; accept AS-ZSTTK6-SET;",
    "afi ipv6.unicast from AS21127 action pref=100; med=0; accept AS-ZSTTK6-SET;",
    "afi ipv6 from AS1213 accept { ::/0 }",
    "afi ipv6.unicast from AS1299 action pref = 200; accept ANY AND NOT {0.0.0.0/0};",
    "afi ipv4.unicast from AS6682 at 109.68.121.1 action pref=65435; med=0; community.append(8226:1102); accept ANY AND {0.0.0.0/0^0-24}",
    "afi ipv4.unicast { from AS174 192.38.7.14 at 192.38.7.75 accept AS174; from AS1835 192.38.7.1 at 192.38.7.75 accept AS-UNIC; from AS2603 192.38.7.50 at 192.38.7.75 accept AS-NORDUNET; from AS2686 192.38.7.8 at 192.38.7.75 accept AS-IGNEMEA; from AS2874 192.38.7.10 at 192.38.7.75 accept AS-GLOBALIPNET; from AS6834 192.38.7.4 at 192.38.7.75 accept AS-KMD; from AS8434 192.38.7.92 at 192.38.7.75 accept AS-TELENOR; from AS9120 192.38.7.46 at 192.38.7.75 accept AS-COHAESIO; from AS9167 192.38.7.49 at 192.38.7.75 accept AS-WEBPARTNER; from AS12552 192.38.7.68 at 192.38.7.75 accept AS-IPO; from AS13030 192.38.7.52 at 192.38.7.75 accept AS-INIT7; from AS15516 192.38.7.64 at 192.38.7.75 accept AS-DK-ARROWHEAD; from AS15598 192.38.7.84 at 192.38.7.75 accept AS-IPX; from AS16095 192.38.7.66 at 192.38.7.75 accept AS-JAYNET; from AS16095 192.38.7.67 at 192.38.7.75 accept AS-JAYNET; from AS16150 192.38.7.43 at 192.38.7.75 accept AS16150:AS-CUSTOMERS; from AS16245 192.38.7.93 at 192.38.7.75 accept AS-NGDC; from AS20618 192.38.7.99 at 192.38.7.75 accept AS-INFOCONNECT; from AS20618 192.38.7.100 at 192.38.7.75 accept AS-INFOCONNECT; from AS25384 192.38.7.97 at 192.38.7.75 accept AS-DMDATADK; from AS25384 192.38.7.98 at 192.38.7.75 accept AS-DMDATADK; from AS28717 192.38.7.82 at 192.38.7.75 accept AS-ZENSYSTEMS; from AS29100 192.38.7.77 at 192.38.7.75 accept AS29100; from AS29266 192.38.7.41 at 192.38.7.75 accept AS-DANMARKSRADIO; from AS31027 192.38.7.58 at 192.38.7.75 accept AS-NIANET; from AS31661 192.38.7.12 at 192.38.7.75 accept AS-COMX; from AS33916 192.38.7.87 at 192.38.7.75 accept AS33916; from AS33926 192.38.7.72 at 192.38.7.75 accept AS-EUROTRANSIT; from AS34823 192.38.7.95 at 192.38.7.75 accept AS34823; from AS41025 192.38.7.28 at 192.38.7.75 accept AS-BUTLERNET; from AS42525 192.38.7.26 at 192.38.7.75 accept AS-GCNET; from AS43457 192.38.7.73 at 192.38.7.75 accept AS-VGDC; }",
    "afi ipv4.unicast, ipv6.unicast from AS2895 action pref=10; accept ANY",
    "afi ipv6.unicast from AS8365 ACCEPT AS-MANDA",
    "afi ipv6.unicast from AS8928 action pref= 10; accept ANY",
    "afi ipv4.unicast from AS3344:PRNG-LONAP action pref=64535; community.append(3344:60000, 3344:60020, 3344:8330); accept ANY AND NOT AS3344:fltr-filterlist",
]

PARSED_MP_IMPORT_EXAMPLES = [
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"mp-peering": ["AS9002"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"mp-peering": ["AS9002"]}, {"mp-peering": ["AS2356"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"actions": ["pref=100"], "mp-peering": ["AS6939"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"actions": ["pref=100"], "mp-peering": ["AS21127"]}],
        "mp-filter": "AS-ZSTTK6-SET",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"actions": ["pref=100", "med=0"], "mp-peering": ["AS21127"]}],
        "mp-filter": "AS-ZSTTK6-SET",
    },
    {
        "afi-list": ["ipv6"],
        "from": [{"mp-peering": ["AS1213"]}],
        "mp-filter": "{ ::/0 }",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"actions": ["pref = 200"], "mp-peering": ["AS1299"]}],
        "mp-filter": "ANY AND NOT {0.0.0.0/0}",
    },
    {
        "afi-list": ["ipv4.unicast"],
        "from": [
            {
                "actions": ["pref=65435", "med=0", "community.append(8226:1102)"],
                "mp-peering": ["AS6682", "at", "109.68.121.1"],
            }
        ],
        "mp-filter": "ANY AND {0.0.0.0/0^0-24}",
    },
    {
        "afi-list": ["ipv4.unicast"],
        "import-factors": [
            {
                "from": [{"mp-peering": ["AS174", "192.38.7.14", "at", "192.38.7.75"]}],
                "mp-filter": "AS174",
            },
            {
                "from": [{"mp-peering": ["AS1835", "192.38.7.1", "at", "192.38.7.75"]}],
                "mp-filter": "AS-UNIC",
            },
            {
                "from": [
                    {"mp-peering": ["AS2603", "192.38.7.50", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NORDUNET",
            },
            {
                "from": [{"mp-peering": ["AS2686", "192.38.7.8", "at", "192.38.7.75"]}],
                "mp-filter": "AS-IGNEMEA",
            },
            {
                "from": [
                    {"mp-peering": ["AS2874", "192.38.7.10", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-GLOBALIPNET",
            },
            {
                "from": [{"mp-peering": ["AS6834", "192.38.7.4", "at", "192.38.7.75"]}],
                "mp-filter": "AS-KMD",
            },
            {
                "from": [
                    {"mp-peering": ["AS8434", "192.38.7.92", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-TELENOR",
            },
            {
                "from": [
                    {"mp-peering": ["AS9120", "192.38.7.46", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-COHAESIO",
            },
            {
                "from": [
                    {"mp-peering": ["AS9167", "192.38.7.49", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-WEBPARTNER",
            },
            {
                "from": [
                    {"mp-peering": ["AS12552", "192.38.7.68", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-IPO",
            },
            {
                "from": [
                    {"mp-peering": ["AS13030", "192.38.7.52", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INIT7",
            },
            {
                "from": [
                    {"mp-peering": ["AS15516", "192.38.7.64", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DK-ARROWHEAD",
            },
            {
                "from": [
                    {"mp-peering": ["AS15598", "192.38.7.84", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-IPX",
            },
            {
                "from": [
                    {"mp-peering": ["AS16095", "192.38.7.66", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-JAYNET",
            },
            {
                "from": [
                    {"mp-peering": ["AS16095", "192.38.7.67", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-JAYNET",
            },
            {
                "from": [
                    {"mp-peering": ["AS16150", "192.38.7.43", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS16150:AS-CUSTOMERS",
            },
            {
                "from": [
                    {"mp-peering": ["AS16245", "192.38.7.93", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NGDC",
            },
            {
                "from": [
                    {"mp-peering": ["AS20618", "192.38.7.99", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INFOCONNECT",
            },
            {
                "from": [
                    {"mp-peering": ["AS20618", "192.38.7.100", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INFOCONNECT",
            },
            {
                "from": [
                    {"mp-peering": ["AS25384", "192.38.7.97", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DMDATADK",
            },
            {
                "from": [
                    {"mp-peering": ["AS25384", "192.38.7.98", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DMDATADK",
            },
            {
                "from": [
                    {"mp-peering": ["AS28717", "192.38.7.82", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-ZENSYSTEMS",
            },
            {
                "from": [
                    {"mp-peering": ["AS29100", "192.38.7.77", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS29100",
            },
            {
                "from": [
                    {"mp-peering": ["AS29266", "192.38.7.41", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DANMARKSRADIO",
            },
            {
                "from": [
                    {"mp-peering": ["AS31027", "192.38.7.58", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NIANET",
            },
            {
                "from": [
                    {"mp-peering": ["AS31661", "192.38.7.12", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-COMX",
            },
            {
                "from": [
                    {"mp-peering": ["AS33916", "192.38.7.87", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS33916",
            },
            {
                "from": [
                    {"mp-peering": ["AS33926", "192.38.7.72", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-EUROTRANSIT",
            },
            {
                "from": [
                    {"mp-peering": ["AS34823", "192.38.7.95", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS34823",
            },
            {
                "from": [
                    {"mp-peering": ["AS41025", "192.38.7.28", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-BUTLERNET",
            },
            {
                "from": [
                    {"mp-peering": ["AS42525", "192.38.7.26", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-GCNET",
            },
            {
                "from": [
                    {"mp-peering": ["AS43457", "192.38.7.73", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-VGDC",
            },
        ],
    },
    {
        "afi-list": ["ipv4.unicast", "ipv6.unicast"],
        "from": [{"actions": ["pref=10"], "mp-peering": ["AS2895"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"mp-peering": ["AS8365"]}],
        "mp-filter": "AS-MANDA",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "from": [{"actions": ["pref= 10"], "mp-peering": ["AS8928"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv4.unicast"],
        "from": [
            {
                "actions": [
                    "pref=64535",
                    "community.append(3344:60000, 3344:60020, 3344:8330)",
                ],
                "mp-peering": ["AS3344:PRNG-LONAP"],
            }
        ],
        "mp-filter": "ANY AND NOT AS3344:fltr-filterlist",
    },
]


def test_mp_import():
    for example, expected in zip(MP_IMPORT_EXAMPLES, PARSED_MP_IMPORT_EXAMPLES):
        success, results = mp_import.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected


MP_PEERING_EXAMPLES = [
    "AS51468",
    "AS9150:AS-PEERS-AMSIX",
    "AS8717 2001:67c:20d0:fffe:ffff:ffff:ffff:fffe at 2001:67c:20d0:fffe:ffff:ffff:ffff:fffd",
    "AS35053 2001:7f8:8:20:0:88ed:0:1 at 2001:7f8:8:20:0:2349:0:fe",
    "AS10310 at AS3326---DEE---mx01-fra1",
    "AS9186:AS-CUSTOMERS AND AS204094",
    "AS-ANY EXCEPT AS5398:AS-AMS-IX-FILTER",
]

PARSED_MP_PEERING_EXAMPLES = [
    {"as-expression": ["AS51468"]},
    {"as-expression": ["AS9150:AS-PEERS-AMSIX"]},
    {
        "as-expression": ["AS8717"],
        "mp-router-expression-1": ["2001:67c:20d0:fffe:ffff:ffff:ffff:fffe"],
        "mp-router-expression-2": ["2001:67c:20d0:fffe:ffff:ffff:ffff:fffd"],
    },
    {
        "as-expression": ["AS35053"],
        "mp-router-expression-1": ["2001:7f8:8:20:0:88ed:0:1"],
        "mp-router-expression-2": ["2001:7f8:8:20:0:2349:0:fe"],
    },
    {
        "as-expression": ["AS10310"],
        "mp-router-expression-2": ["AS3326---DEE---mx01-fra1"],
    },
    {
        "as-expression": ["AS9186:AS-CUSTOMERS", "and", "AS204094"],
    },
    {"as-expression": ["AS-ANY", "except", "AS5398:AS-AMS-IX-FILTER"]},
]


def test_mp_peering():
    for example, expected in zip(MP_PEERING_EXAMPLES, PARSED_MP_PEERING_EXAMPLES):
        success, results = mp_peering.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected

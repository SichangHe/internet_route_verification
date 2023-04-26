from pyparsing import ParseResults

from ..lex import action, mp_filter, mp_import, mp_peering

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
        "mp-peerings": [{"mp-peering": ["AS9002"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"mp-peering": ["AS9002"]}, {"mp-peering": ["AS2356"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref=100"], "mp-peering": ["AS6939"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref=100"], "mp-peering": ["AS21127"]}],
        "mp-filter": "AS-ZSTTK6-SET",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref=100", "med=0"], "mp-peering": ["AS21127"]}],
        "mp-filter": "AS-ZSTTK6-SET",
    },
    {
        "afi-list": ["ipv6"],
        "mp-peerings": [{"mp-peering": ["AS1213"]}],
        "mp-filter": "{ ::/0 }",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref = 200"], "mp-peering": ["AS1299"]}],
        "mp-filter": "ANY AND NOT {0.0.0.0/0}",
    },
    {
        "afi-list": ["ipv4.unicast"],
        "mp-peerings": [
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
                "mp-peerings": [
                    {"mp-peering": ["AS174", "192.38.7.14", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS174",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS1835", "192.38.7.1", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-UNIC",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS2603", "192.38.7.50", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NORDUNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS2686", "192.38.7.8", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-IGNEMEA",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS2874", "192.38.7.10", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-GLOBALIPNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS6834", "192.38.7.4", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-KMD",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS8434", "192.38.7.92", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-TELENOR",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS9120", "192.38.7.46", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-COHAESIO",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS9167", "192.38.7.49", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-WEBPARTNER",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS12552", "192.38.7.68", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-IPO",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS13030", "192.38.7.52", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INIT7",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS15516", "192.38.7.64", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DK-ARROWHEAD",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS15598", "192.38.7.84", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-IPX",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS16095", "192.38.7.66", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-JAYNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS16095", "192.38.7.67", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-JAYNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS16150", "192.38.7.43", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS16150:AS-CUSTOMERS",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS16245", "192.38.7.93", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NGDC",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS20618", "192.38.7.99", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INFOCONNECT",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS20618", "192.38.7.100", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-INFOCONNECT",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS25384", "192.38.7.97", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DMDATADK",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS25384", "192.38.7.98", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DMDATADK",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS28717", "192.38.7.82", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-ZENSYSTEMS",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS29100", "192.38.7.77", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS29100",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS29266", "192.38.7.41", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-DANMARKSRADIO",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS31027", "192.38.7.58", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-NIANET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS31661", "192.38.7.12", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-COMX",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS33916", "192.38.7.87", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS33916",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS33926", "192.38.7.72", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-EUROTRANSIT",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS34823", "192.38.7.95", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS34823",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS41025", "192.38.7.28", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-BUTLERNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS42525", "192.38.7.26", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-GCNET",
            },
            {
                "mp-peerings": [
                    {"mp-peering": ["AS43457", "192.38.7.73", "at", "192.38.7.75"]}
                ],
                "mp-filter": "AS-VGDC",
            },
        ],
    },
    {
        "afi-list": ["ipv4.unicast", "ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref=10"], "mp-peering": ["AS2895"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"mp-peering": ["AS8365"]}],
        "mp-filter": "AS-MANDA",
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"actions": ["pref= 10"], "mp-peering": ["AS8928"]}],
        "mp-filter": "ANY",
    },
    {
        "afi-list": ["ipv4.unicast"],
        "mp-peerings": [
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


MP_EXPORT_EXAMPLES = [
    "afi ipv6.unicast to AS1880 announce AS1881",
    "afi ipv6.unicast to AS3356 announce AS2597:AS-CUSTOMERS-v6",
    "afi ipv4.unicast to AS6802 194.141.252.21 at 194.141.252.22 announce AS5421 AS112;",
    "afi ipv4.unicast to AS6777 action community .= { 6777:6777 }; announce AS9150:AS-INTERCONNECT",
    "afi ipv6.unicast to AS41965 at 2001:4D00:0:1:62:89:0:1 action med=0; announce AS8226 AS8226:AS-CUST",
]

PARSED_MP_EXPORT_EXAMPLES = [
    {
        "afi-list": ["ipv6.unicast"],
        "mp-filter": "AS1881",
        "mp-peerings": [{"mp-peering": ["AS1880"]}],
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-filter": "AS2597:AS-CUSTOMERS-v6",
        "mp-peerings": [{"mp-peering": ["AS3356"]}],
    },
    {
        "afi-list": ["ipv4.unicast"],
        "mp-filter": "AS5421 AS112",
        "mp-peerings": [
            {"mp-peering": ["AS6802", "194.141.252.21", "at", "194.141.252.22"]}
        ],
    },
    {
        "afi-list": ["ipv4.unicast"],
        "mp-filter": "AS9150:AS-INTERCONNECT",
        "mp-peerings": [
            {"actions": ["community .= { 6777:6777 }"], "mp-peering": ["AS6777"]}
        ],
    },
    {
        "afi-list": ["ipv6.unicast"],
        "mp-filter": "AS8226 AS8226:AS-CUST",
        "mp-peerings": [
            {
                "actions": ["med=0"],
                "mp-peering": ["AS41965", "at", "2001:4D00:0:1:62:89:0:1"],
            }
        ],
    },
]


def test_mp_export():
    for example, expected in zip(MP_EXPORT_EXAMPLES, PARSED_MP_EXPORT_EXAMPLES):
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


MP_FILTER_EXAMPLES = [
    "ANY",
    "NOT ANY",
    "{ }",
    "{ 0.0.0.0/0 }",
    "{ 128.9.0.0/16, 128.8.0.0/16, 128.7.128.0/17, 5.0.0.0/8 }",
    "{ 5.0.0.0/8^+, 128.9.0.0/16^-, 30.0.0.0/8^16, 30.0.0.0/8^24-32 }",
    "AS-SAT-TRAKT-V6 AS-SOX",
    "{2001:503:c27::/48, 2001:503:231d::/48}",
    "community(8501:1011, 8501:1013, 8501:1014, 8501:1015, 8501:1016)",
    "(PeerAS OR PeerAS:AS-TO-AIX) AND <^PeerAS+PeerAS:AS-TO-AIX*$>",
    "AS12874 and AS-FASTWEB and AS-FASTWEB-GLOBAL",
    "ANY AND NOT community.contains(8501:1120)",
    "AS26415 {2001:503:c27::/48, 2001:503:231d::/48}",
]

PARSED_MP_FILTER_EXAMPLES = [
    {"policy-filter": [{"path-attribute": "ANY"}]},
    {"not": {"policy-filter": [{"path-attribute": "ANY"}]}},
    {"policy-filter": [{"address-prefix-set": []}]},
    {"policy-filter": [{"address-prefix-set": ["0.0.0.0/0"]}]},
    {
        "policy-filter": [
            {
                "address-prefix-set": [
                    "128.9.0.0/16",
                    "128.8.0.0/16",
                    "128.7.128.0/17",
                    "5.0.0.0/8",
                ]
            }
        ]
    },
    {
        "policy-filter": [
            {
                "address-prefix-set": [
                    "5.0.0.0/8^+",
                    "128.9.0.0/16^-",
                    "30.0.0.0/8^16",
                    "30.0.0.0/8^24-32",
                ]
            }
        ]
    },
    {
        "policy-filter": [
            {"path-attribute": "AS-SAT-TRAKT-V6"},
            {"path-attribute": "AS-SOX"},
        ]
    },
    {
        "policy-filter": [
            {"address-prefix-set": ["2001:503:c27::/48", "2001:503:231d::/48"]}
        ]
    },
    {
        "community": {
            "args": ["8501:1011", "8501:1013", "8501:1014", "8501:1015", "8501:1016"]
        }
    },
    {
        "and": {
            "left": {
                "mp-filter": {
                    "or": {
                        "left": {"policy-filter": [{"path-attribute": "PeerAS"}]},
                        "right": {
                            "policy-filter": [{"path-attribute": "PeerAS:AS-TO-AIX"}]
                        },
                    }
                }
            },
            "right": {
                "policy-filter": [{"path-attribute": "<^PeerAS+PeerAS:AS-TO-AIX*$>"}]
            },
        }
    },
    {
        "and": {
            "left": {"policy-filter": [{"path-attribute": "AS12874"}]},
            "right": {
                "and": {
                    "left": {"policy-filter": [{"path-attribute": "AS-FASTWEB"}]},
                    "right": {
                        "policy-filter": [{"path-attribute": "AS-FASTWEB-GLOBAL"}]
                    },
                }
            },
        }
    },
    {
        "and": {
            "left": {"policy-filter": [{"path-attribute": "ANY"}]},
            "right": {
                "not": {"community": {"method": "contains", "args": ["8501:1120"]}}
            },
        }
    },
    {
        "policy-filter": [
            {"path-attribute": "AS26415"},
            {"address-prefix-set": ["2001:503:c27::/48", "2001:503:231d::/48"]},
        ]
    },
]


def test_mp_filter():
    for example, expected in zip(MP_FILTER_EXAMPLES, PARSED_MP_FILTER_EXAMPLES):
        success, results = mp_filter.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected


MP_FILTER_ILEGAL_EXAMPLES = [
    "ANY ANY NOT AS39326:FLTR-FILTERLIST",
]


def test_mp_filter_fail():
    for example in MP_FILTER_ILEGAL_EXAMPLES:
        assert not mp_filter.matches(example)


ACTION_EXAMPLES = [
    "pref=100",
    "pref = 200",
    "med=0",
    "community.append(8226:1102)",
    "community.append(3344:60000, 3344:60020, 3344:8330)",
    "community .= { 100 }",
    "aspath.prepend(AS1, AS1, AS1)",
]

PARSED_ACTION_EXAMPLES = [
    {"assignment": {"assigned": "100", "assignee": "pref"}},
    {"assignment": {"assigned": "200", "assignee": "pref"}},
    {"assignment": {"assigned": "0", "assignee": "med"}},
    {"community": {"args": ["8226:1102"], "method": "append"}},
    {
        "community": {
            "args": ["3344:60000", "3344:60020", "3344:8330"],
            "method": "append",
        }
    },
    {"add-community": ["100"]},
    {
        "method-call": {
            "args": ["AS1", "AS1", "AS1"],
            "method": "prepend",
            "rp-attribute": "aspath",
        }
    },
]


def test_action():
    for example, expected in zip(ACTION_EXAMPLES, PARSED_ACTION_EXAMPLES):
        success, results = action.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected

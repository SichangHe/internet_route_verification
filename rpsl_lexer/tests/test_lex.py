from pyparsing import ParseResults

from ..lex import action, as_expr, mp_filter, mp_import, mp_peering

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
    "afi any { from AS-ANY action community.delete(64628:10, 64628:11, 64628:12, 64628:13, 64628:14, 64628:15, 64628:20, 64628:21, 64628:22); accept ANY; } REFINE afi any { from AS-ANY action pref = 65535; accept community(65535:0); from AS-ANY action pref = 65435; accept ANY; } REFINE afi any { from AS-ANY accept NOT AS199284^+; } REFINE afi ipv4 { from AS-ANY accept NOT fltr-martian; } REFINE afi ipv4 { from AS-ANY accept { 0.0.0.0/0^0-24 } AND NOT community(65535:666); from AS-ANY accept { 0.0.0.0/0^24-32 } AND community(65535:666); } REFINE afi ipv6 { from AS-ANY accept { 2000::/3^4-48 } AND NOT community(65535:666); from AS-ANY accept { 2000::/3^64-128 } AND community(65535:666); } REFINE afi any { from AS15725 action community .= { 64628:20 }; accept AS-IKS AND <^AS-IKS+$>; from AS196714 action community .= { 64628:20 }; accept AS-TNETKOM AND <^AS-TNETKOM+$>; from AS199284:AS-UP action community .= { 64628:21 }; accept ANY; from AS35366 action community .= { 64628:22 }; accept AS-ISPPRO AND <^AS-ISPPRO+$>; from AS20940 action community .= { 64628:22 }; accept <^AS-AKAMAI+$>; from AS16509 action community .= { 64628:22 }; accept <^AS-AMAZON+$>; from AS32934 action community .= { 64628:22 }; accept <^AS-FACEBOOK+$>; from AS2906 action community .= { 64628:22 }; accept <^AS-NFLX+$>; from AS46489 action community .= { 64628:22 }; accept <^AS-TWITCH+$>; from AS714 action community .= { 64628:22 }; accept <^AS-APPLE+$>; from AS26415 action community .= { 64628:22 }; accept <^AS-GTLD+$>; from AS13335 action community .= { 64628:22 }; accept <^AS-CLOUDFLARE+$>; from AS-ANY action community .= { 64628:22 }; accept PeerAS and <^PeerAS+$>; } REFINE afi any { from AS-ANY EXCEPT (AS40027 OR AS63293 OR AS65535) accept ANY; }",
    "from AS2 action pref = 2; accept AS226; except { from AS3 action pref = 3; accept {128.9.0.0/16}; }",
    "afi any.unicast from AS65001 accept as-foo; except afi any.unicast { from AS65002 accept AS65226; } except afi ipv6.unicast { from AS65003 accept {2001:0DB8::/32}; }",
    "{ from AS-ANY action pref = 1; accept community(3560:10); from AS-ANY action pref = 2; accept community(3560:20); } refine { from AS1 accept AS1; from AS2 accept AS2; from AS3 accept AS3; }",
    "{ from AS-ANY action med = 0; accept {0.0.0.0/0^0-18}; } refine { from AS1 at 7.7.7.1 action pref = 1; accept AS1; from AS1 action pref = 2; accept AS1; }",
    "{ from AS-ANY action community(6774:65231); accept ANY AND NOT AS6774:FLTR-BOGONS; } refine { from AS6774:PRNG-BE-BNIX action community.append(6774:65100); from AS6774:PRNG-DE-DECIX action community.append(6774:65104); from AS6774:PRNG-FR-SFINX action community.append(6774:65102); from AS6774:PRNG-NL-AMSIX action community.append(6774:65101); from AS6774:PRNG-UK-LINX action community.append(6774:65103); accept (PeerAS OR AS6774:AS-PEERS:PeerAS); }",
    "from AS20965 62.40.124.89 action community.append(5408:1001); accept NOT community.contains(5408:1002) AND NOT community.contains(5408:1003) AND NOT fltr-martian; REFINE { from AS20965 action aspath.prepend(AS20965,AS20965,AS20965); accept community.contains(20965:7777); from AS20965 accept ANY; }",
]

LEXED_MP_IMPORT_EXAMPLES = [
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
    {
        "afi-list": ["any"],
        "refine": {
            "left": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {
                                "mp-peering": ["AS-ANY"],
                                "actions": [
                                    "community.delete(64628:10, 64628:11, 64628:12, 64628:13, 64628:14, 64628:15, 64628:20, 64628:21, 64628:22)"
                                ],
                            }
                        ],
                        "mp-filter": "ANY",
                    }
                ]
            },
            "right": {
                "afi-list": ["any"],
                "refine": {
                    "left": {
                        "import-factors": [
                            {
                                "mp-peerings": [
                                    {
                                        "mp-peering": ["AS-ANY"],
                                        "actions": ["pref = 65535"],
                                    }
                                ],
                                "mp-filter": "community(65535:0)",
                            },
                            {
                                "mp-peerings": [
                                    {
                                        "mp-peering": ["AS-ANY"],
                                        "actions": ["pref = 65435"],
                                    }
                                ],
                                "mp-filter": "ANY",
                            },
                        ]
                    },
                    "right": {
                        "afi-list": ["any"],
                        "refine": {
                            "left": {
                                "import-factors": [
                                    {
                                        "mp-peerings": [{"mp-peering": ["AS-ANY"]}],
                                        "mp-filter": "NOT AS199284^+",
                                    }
                                ]
                            },
                            "right": {
                                "afi-list": ["ipv4"],
                                "refine": {
                                    "left": {
                                        "import-factors": [
                                            {
                                                "mp-peerings": [
                                                    {"mp-peering": ["AS-ANY"]}
                                                ],
                                                "mp-filter": "NOT fltr-martian",
                                            }
                                        ]
                                    },
                                    "right": {
                                        "afi-list": ["ipv4"],
                                        "refine": {
                                            "left": {
                                                "import-factors": [
                                                    {
                                                        "mp-peerings": [
                                                            {"mp-peering": ["AS-ANY"]}
                                                        ],
                                                        "mp-filter": "{ 0.0.0.0/0^0-24 } AND NOT community(65535:666)",
                                                    },
                                                    {
                                                        "mp-peerings": [
                                                            {"mp-peering": ["AS-ANY"]}
                                                        ],
                                                        "mp-filter": "{ 0.0.0.0/0^24-32 } AND community(65535:666)",
                                                    },
                                                ]
                                            },
                                            "right": {
                                                "afi-list": ["ipv6"],
                                                "refine": {
                                                    "left": {
                                                        "import-factors": [
                                                            {
                                                                "mp-peerings": [
                                                                    {
                                                                        "mp-peering": [
                                                                            "AS-ANY"
                                                                        ]
                                                                    }
                                                                ],
                                                                "mp-filter": "{ 2000::/3^4-48 } AND NOT community(65535:666)",
                                                            },
                                                            {
                                                                "mp-peerings": [
                                                                    {
                                                                        "mp-peering": [
                                                                            "AS-ANY"
                                                                        ]
                                                                    }
                                                                ],
                                                                "mp-filter": "{ 2000::/3^64-128 } AND community(65535:666)",
                                                            },
                                                        ]
                                                    },
                                                    "right": {
                                                        "afi-list": ["any"],
                                                        "refine": {
                                                            "left": {
                                                                "import-factors": [
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS15725"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:20 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "AS-IKS AND <^AS-IKS+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS196714"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:20 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "AS-TNETKOM AND <^AS-TNETKOM+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS199284:AS-UP"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:21 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "ANY",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS35366"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "AS-ISPPRO AND <^AS-ISPPRO+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS20940"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-AKAMAI+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS16509"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-AMAZON+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS32934"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-FACEBOOK+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS2906"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-NFLX+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS46489"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-TWITCH+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS714"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-APPLE+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS26415"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-GTLD+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS13335"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "<^AS-CLOUDFLARE+$>",
                                                                    },
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS-ANY"
                                                                                ],
                                                                                "actions": [
                                                                                    "community .= { 64628:22 }"
                                                                                ],
                                                                            }
                                                                        ],
                                                                        "mp-filter": "PeerAS and <^PeerAS+$>",
                                                                    },
                                                                ]
                                                            },
                                                            "right": {
                                                                "afi-list": ["any"],
                                                                "import-factors": [
                                                                    {
                                                                        "mp-peerings": [
                                                                            {
                                                                                "mp-peering": [
                                                                                    "AS-ANY",
                                                                                    "EXCEPT",
                                                                                    "(AS40027",
                                                                                    "OR",
                                                                                    "AS63293",
                                                                                    "OR",
                                                                                    "AS65535)",
                                                                                ]
                                                                            }
                                                                        ],
                                                                        "mp-filter": "ANY",
                                                                    }
                                                                ],
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    },
    {
        "except": {
            "left": {
                "mp-peerings": [{"mp-peering": ["AS2"], "actions": ["pref = 2"]}],
                "mp-filter": "AS226",
            },
            "right": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {"mp-peering": ["AS3"], "actions": ["pref = 3"]}
                        ],
                        "mp-filter": "{128.9.0.0/16}",
                    }
                ]
            },
        }
    },
    {
        "afi-list": ["any.unicast"],
        "except": {
            "left": {
                "mp-peerings": [{"mp-peering": ["AS65001"]}],
                "mp-filter": "as-foo",
            },
            "right": {
                "afi-list": ["any.unicast"],
                "except": {
                    "left": {
                        "import-factors": [
                            {
                                "mp-peerings": [{"mp-peering": ["AS65002"]}],
                                "mp-filter": "AS65226",
                            }
                        ]
                    },
                    "right": {
                        "afi-list": ["ipv6.unicast"],
                        "import-factors": [
                            {
                                "mp-peerings": [{"mp-peering": ["AS65003"]}],
                                "mp-filter": "{2001:0DB8::/32}",
                            }
                        ],
                    },
                },
            },
        },
    },
    {
        "refine": {
            "left": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {"mp-peering": ["AS-ANY"], "actions": ["pref = 1"]}
                        ],
                        "mp-filter": "community(3560:10)",
                    },
                    {
                        "mp-peerings": [
                            {"mp-peering": ["AS-ANY"], "actions": ["pref = 2"]}
                        ],
                        "mp-filter": "community(3560:20)",
                    },
                ]
            },
            "right": {
                "import-factors": [
                    {"mp-peerings": [{"mp-peering": ["AS1"]}], "mp-filter": "AS1"},
                    {"mp-peerings": [{"mp-peering": ["AS2"]}], "mp-filter": "AS2"},
                    {"mp-peerings": [{"mp-peering": ["AS3"]}], "mp-filter": "AS3"},
                ]
            },
        }
    },
    {
        "refine": {
            "left": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {"mp-peering": ["AS-ANY"], "actions": ["med = 0"]}
                        ],
                        "mp-filter": "{0.0.0.0/0^0-18}",
                    }
                ]
            },
            "right": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {
                                "mp-peering": ["AS1", "at", "7.7.7.1"],
                                "actions": ["pref = 1"],
                            }
                        ],
                        "mp-filter": "AS1",
                    },
                    {
                        "mp-peerings": [
                            {"mp-peering": ["AS1"], "actions": ["pref = 2"]}
                        ],
                        "mp-filter": "AS1",
                    },
                ]
            },
        }
    },
    {
        "refine": {
            "left": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {
                                "mp-peering": ["AS-ANY"],
                                "actions": ["community(6774:65231)"],
                            }
                        ],
                        "mp-filter": "ANY AND NOT AS6774:FLTR-BOGONS",
                    }
                ]
            },
            "right": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {
                                "mp-peering": ["AS6774:PRNG-BE-BNIX"],
                                "actions": ["community.append(6774:65100)"],
                            },
                            {
                                "mp-peering": ["AS6774:PRNG-DE-DECIX"],
                                "actions": ["community.append(6774:65104)"],
                            },
                            {
                                "mp-peering": ["AS6774:PRNG-FR-SFINX"],
                                "actions": ["community.append(6774:65102)"],
                            },
                            {
                                "mp-peering": ["AS6774:PRNG-NL-AMSIX"],
                                "actions": ["community.append(6774:65101)"],
                            },
                            {
                                "mp-peering": ["AS6774:PRNG-UK-LINX"],
                                "actions": ["community.append(6774:65103)"],
                            },
                        ],
                        "mp-filter": "(PeerAS OR AS6774:AS-PEERS:PeerAS)",
                    }
                ]
            },
        }
    },
    {
        "refine": {
            "left": {
                "mp-peerings": [
                    {
                        "mp-peering": ["AS20965", "62.40.124.89"],
                        "actions": ["community.append(5408:1001)"],
                    }
                ],
                "mp-filter": "NOT community.contains(5408:1002) AND NOT community.contains(5408:1003) AND NOT fltr-martian",
            },
            "right": {
                "import-factors": [
                    {
                        "mp-peerings": [
                            {
                                "mp-peering": ["AS20965"],
                                "actions": ["aspath.prepend(AS20965,AS20965,AS20965)"],
                            }
                        ],
                        "mp-filter": "community.contains(20965:7777)",
                    },
                    {"mp-peerings": [{"mp-peering": ["AS20965"]}], "mp-filter": "ANY"},
                ]
            },
        }
    },
]


def test_mp_import():
    for example, expected in zip(MP_IMPORT_EXAMPLES, LEXED_MP_IMPORT_EXAMPLES):
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

LEXED_MP_EXPORT_EXAMPLES = [
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

MP_DEFAULT_EXAMPLES = [
    "to AS8400",
    "to AS22351 action pref=100; networks ANY",
    "to AS8732 action pref=100;",
    "afi ipv6.unicast to AS12502 action pref=100; networks ANY",
]

LEXED_MP_DEFAULT_EXAMPLES = [
    {"mp-peerings": [{"mp-peering": ["AS8400"]}]},
    {
        "mp-peerings": [{"mp-peering": ["AS22351"], "actions": ["pref=100"]}],
        "mp-filter": "ANY",
    },
    {"mp-peerings": [{"mp-peering": ["AS8732"], "actions": ["pref=100"]}]},
    {
        "afi-list": ["ipv6.unicast"],
        "mp-peerings": [{"mp-peering": ["AS12502"], "actions": ["pref=100"]}],
        "mp-filter": "ANY",
    },
]


def test_mp_default():
    for example, expected in zip(MP_DEFAULT_EXAMPLES, LEXED_MP_DEFAULT_EXAMPLES):
        success, results = mp_import.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected


def test_mp_export():
    for example, expected in zip(MP_EXPORT_EXAMPLES, LEXED_MP_EXPORT_EXAMPLES):
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
    "(AS42 or AS3856)",
    "AS28788 80.249.208.237",
    "AS-ANY except (AS40027 or AS63293 or AS65535)",
]

LEXED_MP_PEERING_EXAMPLES = [
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
    {"as-expression": ["(AS42", "or", "AS3856)"]},
    {"as-expression": ["AS28788"], "mp-router-expression-1": ["80.249.208.237"]},
    {
        "as-expression": [
            "AS-ANY",
            "except",
            "(AS40027",
            "or",
            "AS63293",
            "or",
            "AS65535)",
        ]
    },
]


def test_mp_peering():
    for example, expected in zip(MP_PEERING_EXAMPLES, LEXED_MP_PEERING_EXAMPLES):
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
    "ANY ANY NOT AS39326:FLTR-FILTERLIST",
    "{0.0.0.0/0} AND {::/0}",
]

LEXED_MP_FILTER_EXAMPLES = [
    {"filter": "ANY"},
    {"not": {"filter": "ANY"}},
    {"address-prefix-set": []},
    {"address-prefix-set": ["0.0.0.0/0"]},
    {
        "address-prefix-set": [
            "128.9.0.0/16",
            "128.8.0.0/16",
            "128.7.128.0/17",
            "5.0.0.0/8",
        ]
    },
    {
        "address-prefix-set": [
            "5.0.0.0/8^+",
            "128.9.0.0/16^-",
            "30.0.0.0/8^16",
            "30.0.0.0/8^24-32",
        ]
    },
    {"or": {"left": {"filter": "AS-SAT-TRAKT-V6"}, "right": {"filter": "AS-SOX"}}},
    {"address-prefix-set": ["2001:503:c27::/48", "2001:503:231d::/48"]},
    {
        "community": {
            "args": ["8501:1011", "8501:1013", "8501:1014", "8501:1015", "8501:1016"]
        }
    },
    {
        "and": {
            "left": {
                "group": {
                    "or": {
                        "left": {"filter": "PeerAS"},
                        "right": {"filter": "PeerAS:AS-TO-AIX"},
                    }
                }
            },
            "right": {"regex": "^PeerAS+PeerAS:AS-TO-AIX*$"},
        }
    },
    {
        "and": {
            "left": {"filter": "AS12874"},
            "right": {
                "and": {
                    "left": {"filter": "AS-FASTWEB"},
                    "right": {"filter": "AS-FASTWEB-GLOBAL"},
                }
            },
        }
    },
    {
        "and": {
            "left": {"filter": "ANY"},
            "right": {
                "not": {"community": {"method": "contains", "args": ["8501:1120"]}}
            },
        }
    },
    {
        "or": {
            "left": {"filter": "AS26415"},
            "right": {
                "address-prefix-set": ["2001:503:c27::/48", "2001:503:231d::/48"]
            },
        }
    },
    {
        "or": {
            "left": {"filter": "ANY"},
            "right": {
                "or": {
                    "left": {"filter": "ANY"},
                    "right": {"not": {"filter": "AS39326:FLTR-FILTERLIST"}},
                }
            },
        }
    },
    {
        "and": {
            "left": {"address-prefix-set": ["0.0.0.0/0"]},
            "right": {"address-prefix-set": ["::/0"]},
        }
    },
]


def test_mp_filter():
    for example, expected in zip(MP_FILTER_EXAMPLES, LEXED_MP_FILTER_EXAMPLES):
        success, results = mp_filter.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected


ACTION_EXAMPLES = [
    "pref=100",
    "pref = 200",
    "med=0",
    "community.append(8226:1102)",
    "community.append(3344:60000, 3344:60020, 3344:8330)",
    "community .= { 100 }",
    "aspath.prepend(AS1, AS1, AS1)",
    "community = {29222:1000, 29222:1001, 29222:559}",
]

LEXED_ACTION_EXAMPLES = [
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
    {
        "assignment": {
            "assigned-set": ["29222:1000", "29222:1001", "29222:559"],
            "assignee": "community",
        }
    },
]


def test_action():
    for example, expected in zip(ACTION_EXAMPLES, LEXED_ACTION_EXAMPLES):
        success, results = action.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected


AS_EXPR_EXAMPLES = [
    "AS51468",
    "AS9186:AS-CUSTOMERS AND AS204094",
    "AS-ANY EXCEPT AS5398:AS-AMS-IX-FILTER",
    "(AS42 or AS3856)",
    "AS-ANY except (AS40027 or AS63293 or AS65535)",
]

LEXED_AS_EXPR_EXAMPLES = [
    {"field": "AS51468"},
    {"and": {"left": {"field": "AS9186:AS-CUSTOMERS"}, "right": {"field": "AS204094"}}},
    {
        "except": {
            "left": {"field": "AS-ANY"},
            "right": {"field": "AS5398:AS-AMS-IX-FILTER"},
        }
    },
    {"group": {"or": {"left": {"field": "AS42"}, "right": {"field": "AS3856"}}}},
    {
        "except": {
            "left": {"field": "AS-ANY"},
            "right": {
                "group": {
                    "or": {
                        "left": {"field": "AS40027"},
                        "right": {
                            "or": {
                                "left": {"field": "AS63293"},
                                "right": {"field": "AS65535"},
                            }
                        },
                    }
                }
            },
        }
    },
]


def test_as_expr():
    for example, expected in zip(AS_EXPR_EXAMPLES, LEXED_AS_EXPR_EXAMPLES):
        success, results = as_expr.run_tests(example, full_dump=False)
        assert success
        result = results[0][1]
        assert isinstance(result, ParseResults)
        assert result.as_dict() == expected

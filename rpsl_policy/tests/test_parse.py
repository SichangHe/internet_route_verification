from ..parse import import_export
from .test_lex import LEXED_MP_IMPORT_EXAMPLES

PARSED_MP_IMPORT_EXAMPLES = [
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [{"mp_peering": {"as_expr": "AS9002"}}],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {"mp_peering": {"as_expr": "AS9002"}},
                        {"mp_peering": {"as_expr": "AS2356"}},
                    ],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {"as_expr": "AS6939"},
                            "actions": {"pref": "100"},
                        }
                    ],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {"as_expr": "AS21127"},
                            "actions": {"pref": "100"},
                        }
                    ],
                    "mp_filter": ["AS-ZSTTK6-SET"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {"as_expr": "AS21127"},
                            "actions": {"pref": "100", "med": "0"},
                        }
                    ],
                    "mp_filter": ["AS-ZSTTK6-SET"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "any": [
                {
                    "mp_peerings": [{"mp_peering": {"as_expr": "AS1213"}}],
                    "mp_filter": [["::/0"]],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {"as_expr": "AS1299"},
                            "actions": {"pref": "200"},
                        }
                    ],
                    "mp_filter": {
                        "and": {"left": ["ANY"], "right": {"not": [["0.0.0.0/0"]]}}
                    },
                }
            ]
        }
    },
    {
        "ipv4": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS6682",
                                "router_expr2": "109.68.121.1",
                            },
                            "actions": {
                                "pref": "65435",
                                "med": "0",
                                "community": [
                                    {"method": "append", "args": ["8226:1102"]}
                                ],
                            },
                        }
                    ],
                    "mp_filter": {
                        "and": {"left": ["ANY"], "right": [["0.0.0.0/0^0-24"]]}
                    },
                }
            ]
        }
    },
    {
        "ipv4": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS174",
                                "router_expr1": "192.38.7.14",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS174"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS1835",
                                "router_expr1": "192.38.7.1",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-UNIC"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS2603",
                                "router_expr1": "192.38.7.50",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-NORDUNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS2686",
                                "router_expr1": "192.38.7.8",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-IGNEMEA"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS2874",
                                "router_expr1": "192.38.7.10",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-GLOBALIPNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS6834",
                                "router_expr1": "192.38.7.4",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-KMD"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS8434",
                                "router_expr1": "192.38.7.92",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-TELENOR"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS9120",
                                "router_expr1": "192.38.7.46",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-COHAESIO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS9167",
                                "router_expr1": "192.38.7.49",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-WEBPARTNER"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS12552",
                                "router_expr1": "192.38.7.68",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-IPO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS13030",
                                "router_expr1": "192.38.7.52",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-INIT7"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS15516",
                                "router_expr1": "192.38.7.64",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-DK-ARROWHEAD"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS15598",
                                "router_expr1": "192.38.7.84",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-IPX"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS16095",
                                "router_expr1": "192.38.7.66",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-JAYNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS16095",
                                "router_expr1": "192.38.7.67",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-JAYNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS16150",
                                "router_expr1": "192.38.7.43",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS16150:AS-CUSTOMERS"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS16245",
                                "router_expr1": "192.38.7.93",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-NGDC"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS20618",
                                "router_expr1": "192.38.7.99",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-INFOCONNECT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS20618",
                                "router_expr1": "192.38.7.100",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-INFOCONNECT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS25384",
                                "router_expr1": "192.38.7.97",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-DMDATADK"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS25384",
                                "router_expr1": "192.38.7.98",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-DMDATADK"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS28717",
                                "router_expr1": "192.38.7.82",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-ZENSYSTEMS"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS29100",
                                "router_expr1": "192.38.7.77",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS29100"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS29266",
                                "router_expr1": "192.38.7.41",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-DANMARKSRADIO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS31027",
                                "router_expr1": "192.38.7.58",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-NIANET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS31661",
                                "router_expr1": "192.38.7.12",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-COMX"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS33916",
                                "router_expr1": "192.38.7.87",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS33916"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS33926",
                                "router_expr1": "192.38.7.72",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-EUROTRANSIT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS34823",
                                "router_expr1": "192.38.7.95",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS34823"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS41025",
                                "router_expr1": "192.38.7.28",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-BUTLERNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS42525",
                                "router_expr1": "192.38.7.26",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-GCNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS43457",
                                "router_expr1": "192.38.7.73",
                                "router_expr2": "192.38.7.75",
                            }
                        }
                    ],
                    "mp_filter": ["AS-VGDC"],
                },
            ]
        }
    },
    {
        "any": {
            "unicast": [
                {
                    "mp_peerings": [
                        {"mp_peering": {"as_expr": "AS2895"}, "actions": {"pref": "10"}}
                    ],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [{"mp_peering": {"as_expr": "AS8365"}}],
                    "mp_filter": ["AS-MANDA"],
                }
            ]
        }
    },
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [
                        {"mp_peering": {"as_expr": "AS8928"}, "actions": {"pref": "10"}}
                    ],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
    {
        "ipv4": {
            "unicast": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {"as_expr": "AS3344:PRNG-LONAP"},
                            "actions": {
                                "pref": "64535",
                                "community": [
                                    {
                                        "method": "append",
                                        "args": [
                                            "3344:60000",
                                            "3344:60020",
                                            "3344:8330",
                                        ],
                                    }
                                ],
                            },
                        }
                    ],
                    "mp_filter": {
                        "and": {
                            "left": ["ANY"],
                            "right": {"not": ["AS3344:fltr-filterlist"]},
                        }
                    },
                }
            ]
        }
    },
    {
        "any": {
            "any": [
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as_expr": "AS-ANY except (AS40027 or AS63293 or AS65535)"
                            }
                        }
                    ],
                    "mp_filter": ["ANY"],
                }
            ]
        }
    },
]


def test_parse_mp_import():
    for lexed, expected in zip(LEXED_MP_IMPORT_EXAMPLES, PARSED_MP_IMPORT_EXAMPLES):
        result = import_export(lexed, {})
        assert result == expected

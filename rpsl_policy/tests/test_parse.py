from ..parse import import_export
from .test_lex import LEXED_MP_IMPORT_EXAMPLES

PARSED_MP_IMPORT_EXAMPLES = [
    {
        "ipv6": {
            "unicast": [
                {
                    "mp_peerings": [{"mp_peering": {"as-expression": ["AS9002"]}}],
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
                        {"mp_peering": {"as-expression": ["AS9002"]}},
                        {"mp_peering": {"as-expression": ["AS2356"]}},
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
                            "mp_peering": {"as-expression": ["AS6939"]},
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
                            "mp_peering": {"as-expression": ["AS21127"]},
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
                            "mp_peering": {"as-expression": ["AS21127"]},
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
                    "mp_peerings": [{"mp_peering": {"as-expression": ["AS1213"]}}],
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
                            "mp_peering": {"as-expression": ["AS1299"]},
                            "actions": {"pref": "200"},
                        }
                    ],
                    "mp_filter": {
                        "and": {
                            "left": ["ANY"],
                            "right": {"not": [["0.0.0.0/0"]]},
                        }
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
                            "mp_peering": {"as-expression": ["AS6682at109.68.121.1"]},
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
                                "as-expression": ["AS174192.38.7.14at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS174"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS1835192.38.7.1at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-UNIC"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS2603192.38.7.50at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-NORDUNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS2686192.38.7.8at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-IGNEMEA"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS2874192.38.7.10at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-GLOBALIPNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS6834192.38.7.4at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-KMD"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS8434192.38.7.92at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-TELENOR"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS9120192.38.7.46at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-COHAESIO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS9167192.38.7.49at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-WEBPARTNER"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS12552192.38.7.68at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-IPO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS13030192.38.7.52at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-INIT7"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS15516192.38.7.64at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-DK-ARROWHEAD"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS15598192.38.7.84at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-IPX"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS16095192.38.7.66at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-JAYNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS16095192.38.7.67at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-JAYNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS16150192.38.7.43at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS16150:AS-CUSTOMERS"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS16245192.38.7.93at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-NGDC"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS20618192.38.7.99at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-INFOCONNECT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS20618192.38.7.100at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-INFOCONNECT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS25384192.38.7.97at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-DMDATADK"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS25384192.38.7.98at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-DMDATADK"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS28717192.38.7.82at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-ZENSYSTEMS"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS29100192.38.7.77at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS29100"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS29266192.38.7.41at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-DANMARKSRADIO"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS31027192.38.7.58at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-NIANET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS31661192.38.7.12at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-COMX"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS33916192.38.7.87at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS33916"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS33926192.38.7.72at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-EUROTRANSIT"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS34823192.38.7.95at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS34823"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS41025192.38.7.28at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-BUTLERNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS42525192.38.7.26at192.38.7.75"]
                            }
                        }
                    ],
                    "mp_filter": ["AS-GCNET"],
                },
                {
                    "mp_peerings": [
                        {
                            "mp_peering": {
                                "as-expression": ["AS43457192.38.7.73at192.38.7.75"]
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
                        {
                            "mp_peering": {"as-expression": ["AS2895"]},
                            "actions": {"pref": "10"},
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
                    "mp_peerings": [{"mp_peering": {"as-expression": ["AS8365"]}}],
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
                        {
                            "mp_peering": {"as-expression": ["AS8928"]},
                            "actions": {"pref": "10"},
                        }
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
                            "mp_peering": {"as-expression": ["AS3344:PRNG-LONAP"]},
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
                                "as-expression": [
                                    "AS-ANYEXCEPT(AS40027ORAS63293ORAS65535)"
                                ]
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

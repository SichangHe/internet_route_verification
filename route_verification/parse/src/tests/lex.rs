use ::lex::test_util::expected_ast;
use maplit::btreemap;
use net_literals::ip;

use super::*;
use {
    Action::Assigned,
    AsExpr::Single,
    AsName::Num,
    Filter::{Any, AsNum},
    RangeOperator::*,
    RouteSetMember::RSRange,
    RouterExpr::*,
};

#[test]
fn parse_name() {
    assert_eq!(parse_aut_num_name("AS2340").unwrap(), 2340);
    assert!(parse_aut_num_name("AS2340 ").is_err());
    assert!(parse_aut_num_name("AS-2340").is_err());
    assert!(parse_aut_num_name("jfwoe").is_err());
}

#[test]
fn parse_ast() {
    let lexed = expected_ast();
    let (
        Ir {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        },
        counts,
    ) = parse_lexed(lexed);
    assert_eq!(aut_nums, expected_aut_nums());
    assert_eq!(as_sets, expected_as_sets());
    assert_eq!(route_sets, expected_route_sets());
    assert_eq!(peering_sets, expected_peering_sets());
    assert_eq!(filter_sets, expected_filter_sets());
    assert_eq!(as_routes, expected_as_routes());
    let expected_counts = Counts::default();
    assert_eq!(counts, expected_counts);
}

fn expected_aut_nums() -> BTreeMap<u32, AutNum> {
    let body = "remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into();
    let imports = Versions {
        any: Casts {
            any: vec![
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(690)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {"pref".into() => Assigned("100".into())},
                    }],
                    mp_filter: Any,
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(513)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {"pref".into() => Assigned("100".into())},
                    }],
                    mp_filter: Any,
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(559)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {"pref".into() => Assigned("100".into())},
                    }],
                    mp_filter: AsNum(559, NoOp),
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(697)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {"pref".into() => Assigned("100".into())},
                    }],
                    mp_filter: AsNum(697, NoOp),
                },
            ],
            unicast: vec![],
            multicast: vec![],
        },
        ipv4: Casts {
            any: vec![],
            unicast: vec![],
            multicast: vec![],
        },
        ipv6: Casts {
            any: vec![],
            unicast: vec![],
            multicast: vec![],
        },
    };

    let exports = Versions {
        any: Casts {
            any: vec![
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(690)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {},
                    }],
                    mp_filter: AsNum(590, NoOp),
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(513)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {},
                    }],
                    mp_filter: AsNum(590, NoOp),
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(559)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {},
                    }],
                    mp_filter: AsNum(590, NoOp),
                },
                Entry {
                    mp_peerings: vec![PeeringAction {
                        mp_peering: Peering {
                            remote_as: Single(Num(697)),
                            remote_router: None,
                            local_router: None,
                        },
                        actions: btreemap! {},
                    }],
                    mp_filter: AsNum(590, NoOp),
                },
            ],
            unicast: vec![],
            multicast: vec![],
        },
        ipv4: Casts::default(),
        ipv6: Casts::default(),
    };

    BTreeMap::from([(
        590,
        AutNum {
            body,
            imports,
            exports,
        },
    )])
}

fn expected_as_sets() -> BTreeMap<String, AsSet> {
    btreemap! {"AS-RESTENA".into()=> AsSet { body: "descr: Reseau Teleinformatique de l'Education Nationale\ndescr: Educational and research network for Luxembourg\nmembers: AS2602\nmembers: AS42909\nmembers: AS51966\nmembers: AS-LXP\nmembers: AS-VDL\ntech-c: DUMY-RIPE\nadmin-c: DUMY-RIPE\nnotify: noc@restena.lu\nmnt-by: AS2602-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2022-09-08T09:11:41Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec![2602, 42909, 51966], set_members: vec!["AS-LXP".into(), "AS-VDL".into()], is_any: false }}
}

fn expected_route_sets() -> BTreeMap<String, RouteSet> {
    btreemap! {"AS13646:RS-PEERLANS".into()=> RouteSet { body: "descr: Internet Exchange Peering LAN Routes\nmembers: 195.66.224.0/23\nmembers: 194.68.129.0/24\nmembers: 217.29.66.0/23\nmembers: 193.149.1.0/25\nmembers: 193.149.1.128/25\nmembers: 193.148.15.0/24\nmembers: 194.31.232.0/24\nmembers: 194.42.48.0/25\nmembers: 194.53.172.0/26\nmembers: 193.203.0.0/24\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: ZIGGO-SERVICES-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2020-01-21T15:43:54Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec![RSRange(AddrPfxRange { address_prefix: ipn("195.66.224.0/23"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("194.68.129.0/24"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("217.29.66.0/23"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("193.149.1.0/25"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("193.149.1.128/25"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("193.148.15.0/24"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("194.31.232.0/24"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("194.42.48.0/25"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("194.53.172.0/26"), range_operator: NoOp }), RSRange(AddrPfxRange { address_prefix: ipn("193.203.0.0/24"), range_operator: NoOp })] }, "RS-PEERING-TESTBED".into()=> RouteSet { body: "".into(), members: vec![RSRange(AddrPfxRange { address_prefix: ipn("184.164.224.0/19"), range_operator: Range(19, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("204.9.168.0/22"), range_operator: Range(22, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("138.185.228.0/22"), range_operator: Range(22, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("2804:269c::/32"), range_operator: Range(32, 32) }), RSRange(AddrPfxRange { address_prefix: ipn("2804:269c::/42"), range_operator: Range(44, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2804:269c:fe00::/43"), range_operator: Range(48, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2804:269c:fe40::/43"), range_operator: Range(48, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("66.180.190.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("208.87.172.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("199.36.157.0/24"), range_operator: Range(24, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("103.171.218.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("151.216.4.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("102.218.96.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("165.140.104.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("201.219.252.0/23"), range_operator: Range(23, 24) }), RSRange(AddrPfxRange { address_prefix: ipn("2604:4540::/48"), range_operator: Range(48, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2604:4540:80::/44"), range_operator: Range(44, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2620:33:c000::/48"), range_operator: Range(48, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2001:df7:5380::/47"), range_operator: Range(47, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2001:7fc:2::/47"), range_operator: Range(47, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2001:43f8:df0::/47"), range_operator: Range(47, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2620:9e:6000::/47"), range_operator: Range(47, 48) }), RSRange(AddrPfxRange { address_prefix: ipn("2801:1e:1800::/47"), range_operator: Range(47, 48) })] }}
}

fn ipn(s: &str) -> IpNet {
    s.parse().unwrap()
}

fn expected_peering_sets() -> BTreeMap<String, PeeringSet> {
    btreemap! {"AS8785:prng-nyiix".into()=> PeeringSet { body: "descr: NYIIX Peering Partners\npeering: AS2516 at 198.32.160.25\npeering: AS3257 at 198.32.160.29\npeering: AS4323 at 198.32.160.35\npeering: AS4436 at 198.32.160.53\npeering: AS4513 at 198.32.160.32\npeering: AS5496 at 198.32.160.16\npeering: AS6427 at 198.32.160.12\npeering: AS6461 at 198.32.160.22\npeering: AS6660 at 198.32.160.13\npeering: AS6667 at 198.32.160.41\npeering: AS6939 at 198.32.160.61\npeering: AS8001 at 198.32.160.20\npeering: AS8002 at 198.32.160.33\npeering: AS8220 at 198.32.160.34\npeering: AS8647 at 198.32.160.14\npeering: AS8966 at 198.32.160.45\npeering: AS9156 at 198.32.160.24\npeering: AS13768 at 198.32.160.65\npeering: AS13945 at 198.32.160.37\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: MISTRALNOC\ncreated: 2001-10-20T18:41:03Z\nlast-modified: 2005-10-10T11:47:35Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), peerings: vec![Peering { remote_as: Single(Num(2516)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.25"))) }, Peering { remote_as: Single(Num(3257)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.29"))) }, Peering { remote_as: Single(Num(4323)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.35"))) }, Peering { remote_as: Single(Num(4436)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.53"))) }, Peering { remote_as: Single(Num(4513)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.32"))) }, Peering { remote_as: Single(Num(5496)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.16"))) }, Peering { remote_as: Single(Num(6427)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.12"))) }, Peering { remote_as: Single(Num(6461)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.22"))) }, Peering { remote_as: Single(Num(6660)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.13"))) }, Peering { remote_as: Single(Num(6667)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.41"))) }, Peering { remote_as: Single(Num(6939)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.61"))) }, Peering { remote_as: Single(Num(8001)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.20"))) }, Peering { remote_as: Single(Num(8002)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.33"))) }, Peering { remote_as: Single(Num(8220)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.34"))) }, Peering { remote_as: Single(Num(8647)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.14"))) }, Peering { remote_as: Single(Num(8966)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.45"))) }, Peering { remote_as: Single(Num(9156)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.24"))) }, Peering { remote_as: Single(Num(13768)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.65"))) }, Peering { remote_as: Single(Num(13945)), remote_router: None, local_router: Some(Ip(ip!("198.32.160.37"))) }] }}
}

fn expected_filter_sets() -> BTreeMap<String, FilterSet> {
    use crate::{Filter::*, FilterSet};

    btreemap! {"FLTR-EUX".into()=> FilterSet { body: "filter: AS8785 AND AS13285\ndescr: test filter set 1\nmnt-by: MISTRALNOC\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\ncreated: 2002-08-23T22:50:47Z\nlast-modified: 2005-10-10T11:47:30Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), filters: vec![And { left: Box::new(AsNum(8785, NoOp)), right: Box::new(AsNum(13285, NoOp)) }] }}
}

fn expected_as_routes() -> BTreeMap<u32, Vec<IpNet>> {
    btreemap! {10=> vec![ipn("45.143.208.0/22")]}
}

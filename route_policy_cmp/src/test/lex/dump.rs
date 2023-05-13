use std::collections::BTreeMap;

use crate::lex::{
    action::Action::*,
    dump::Dump,
    filter::{Filter::*, Policy::*},
    mp_import::{Casts, Entry, PeeringAction, Versions},
    peering::{AsExpr::*, Peering},
    rpsl_object::{AsOrRouteSet, AutNum},
};

pub use super::*;

const DUMP: &str = r#"{"aut_nums":[{"name":"AS590","body":"remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"as_expr":"AS690"},"actions":{"pref":"100"}}],"mp_filter":["ANY"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS513"},"actions":{"pref":"100"}}],"mp_filter":["ANY"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS559"},"actions":{"pref":"100"}}],"mp_filter":["AS559"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS697"},"actions":{"pref":"100"}}],"mp_filter":["AS697"]}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"as_expr":"AS690"}}],"mp_filter":["AS590"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS513"}}],"mp_filter":["AS590"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS559"}}],"mp_filter":["AS590"]},{"mp_peerings":[{"mp_peering":{"as_expr":"AS697"}}],"mp_filter":["AS590"]}]}}}],"as_sets":[{"name":"AS-RESTENA","body":"descr: Reseau Teleinformatique de l'Education Nationale\ndescr: Educational and research network for Luxembourg\nmembers: AS2602\nmembers: AS42909\nmembers: AS51966\nmembers: AS-LXP\nmembers: AS-VDL\ntech-c: DUMY-RIPE\nadmin-c: DUMY-RIPE\nnotify: noc@restena.lu\nmnt-by: AS2602-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2022-09-08T09:11:41Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","members":["AS2602","AS42909","AS51966","AS-LXP","AS-VDL"]}],"route_sets":[{"name":"AS13646:RS-PEERLANS","body":"descr: Internet Exchange Peering LAN Routes\nmembers: 195.66.224.0/23\nmembers: 194.68.129.0/24\nmembers: 217.29.66.0/23\nmembers: 193.149.1.0/25\nmembers: 193.149.1.128/25\nmembers: 193.148.15.0/24\nmembers: 194.31.232.0/24\nmembers: 194.42.48.0/25\nmembers: 194.53.172.0/26\nmembers: 193.203.0.0/24\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: ZIGGO-SERVICES-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2020-01-21T15:43:54Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","members":["195.66.224.0/23","194.68.129.0/24","217.29.66.0/23","193.149.1.0/25","193.149.1.128/25","193.148.15.0/24","194.31.232.0/24","194.42.48.0/25","194.53.172.0/26","193.203.0.0/24"]}]}"#;

#[test]
fn dump() -> Result<()> {
    let lexed: Dump = serde_json::from_str(DUMP)?;
    let expected = expected_dump();
    assert_eq!(lexed, expected);
    Ok(())
}

fn expected_dump() -> Dump {
    Dump { aut_nums: vec![AutNum { name: "AS590".into(), body: "remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), imports: Versions { any: Some(Casts { any: Some(vec![Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS690".into()), router_expr1: None, router_expr2: None }, actions: Some(BTreeMap::from([("pref".into(), Assigned("100".into()))])) }], mp_filter: Policies(vec![PathAttr("ANY".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS513".into()), router_expr1: None, router_expr2: None }, actions: Some(BTreeMap::from([("pref".into(), Assigned("100".into()))])) }], mp_filter: Policies(vec![PathAttr("ANY".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS559".into()), router_expr1: None, router_expr2: None }, actions: Some(BTreeMap::from([("pref".into(), Assigned("100".into()))])) }], mp_filter: Policies(vec![PathAttr("AS559".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS697".into()), router_expr1: None, router_expr2: None }, actions: Some(BTreeMap::from([("pref".into(), Assigned("100".into()))])) }], mp_filter: Policies(vec![PathAttr("AS697".into())]) }]), unicast: None, multicast: None }), ipv4: None, ipv6: None }, exports: Versions { any: Some(Casts { any: Some(vec![Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS690".into()), router_expr1: None, router_expr2: None }, actions: None }], mp_filter: Policies(vec![PathAttr("AS590".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS513".into()), router_expr1: None, router_expr2: None }, actions: None }], mp_filter: Policies(vec![PathAttr("AS590".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS559".into()), router_expr1: None, router_expr2: None }, actions: None }], mp_filter: Policies(vec![PathAttr("AS590".into())]) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS697".into()), router_expr1: None, router_expr2: None }, actions: None }], mp_filter: Policies(vec![PathAttr("AS590".into())]) }]), unicast: None, multicast: None }), ipv4: None, ipv6: None } }], as_sets: vec![AsOrRouteSet { name: "AS-RESTENA".into(), body: "descr: Reseau Teleinformatique de l'Education Nationale\ndescr: Educational and research network for Luxembourg\nmembers: AS2602\nmembers: AS42909\nmembers: AS51966\nmembers: AS-LXP\nmembers: AS-VDL\ntech-c: DUMY-RIPE\nadmin-c: DUMY-RIPE\nnotify: noc@restena.lu\nmnt-by: AS2602-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2022-09-08T09:11:41Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec!["AS2602".into(), "AS42909".into(), "AS51966".into(), "AS-LXP".into(), "AS-VDL".into()] }], route_sets: vec![AsOrRouteSet { name: "AS13646:RS-PEERLANS".into(), body: "descr: Internet Exchange Peering LAN Routes\nmembers: 195.66.224.0/23\nmembers: 194.68.129.0/24\nmembers: 217.29.66.0/23\nmembers: 193.149.1.0/25\nmembers: 193.149.1.128/25\nmembers: 193.148.15.0/24\nmembers: 194.31.232.0/24\nmembers: 194.42.48.0/25\nmembers: 194.53.172.0/26\nmembers: 193.203.0.0/24\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: ZIGGO-SERVICES-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2020-01-21T15:43:54Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec!["195.66.224.0/23".into(), "194.68.129.0/24".into(), "217.29.66.0/23".into(), "193.149.1.0/25".into(), "193.149.1.128/25".into(), "193.148.15.0/24".into(), "194.31.232.0/24".into(), "194.42.48.0/25".into(), "194.53.172.0/26".into(), "193.203.0.0/24".into()] }] }
}

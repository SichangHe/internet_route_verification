use crate::{Report::*, ReportItem::*, *};
use dashmap::DashMap;
use maplit::hashmap;

use super::*;

const IR: &str = r#"{"aut_nums":{"2914":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":50472}}}}],"mp_filter":{"AsSet":["AS-CHAOS","NoOp"]}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL","NoOp"]}}]},"ipv6":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL-v6","NoOp"]}}]}}},"9583":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}},"actions":{"pref":"20"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4637}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}}]}}},"18106":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}},"actions":{"pref":"100"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}}]}}},"196844":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-AMS-IX-PEERS"}}}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":29414}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Bialystok-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12618}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Bydgoszcz-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25084}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Czestochowa-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15396}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-ICM-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":30778}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Kielce-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":28797}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Koszalin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8323}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Krakow-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16283}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-LODMAN-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12346}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Lublin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8308}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-NASK-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21064}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Olsztyn-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25584}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Opole-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8364}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-POZMAN-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":34604}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Pulawy-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16263}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Radom$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":39873}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Rzeszow-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15744}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Slask-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13119}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Szczecin-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12831}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-TASK-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":35686}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Torun-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15851}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Wroclaw-COM$"}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13065}}},"actions":{"pref":"100"}}],"mp_filter":{"AsPathRE":"AS196844:AS-Zielona_Gora-COM$"}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":29414}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12618}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25084}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15396}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":30778}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":28797}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8323}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16283}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12346}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8308}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21064}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":25584}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":8364}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":34604}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":16263}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":39873}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15744}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13119}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":12831}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":35686}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":15851}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":13065}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":21021}}}}],"mp_filter":{"Not":{"AsSet":["AS196844:AS-AMSIX","NoOp"]}}}]}}}},"as_sets":{},"route_sets":{},"peering_sets":{},"filter_sets":{},"as_routes":{}}"#;
const LINES: [&str;1] = [
    "TABLE_DUMP2|1687212000|B|147.28.7.1|3130|1.6.165.0/24|3130 1239 2914 9583|IGP|147.28.7.1|0|0|1239:321 1239:1000 1239:1010|NAG||"
];

#[test]
fn err_only_checks() -> Result<()> {
    let query = query()?;
    for (expected, line) in expected_err_only_reports().into_iter().zip(LINES) {
        let mut compare = Compare::with_line_dump(line)?;
        compare.verbosity = Verbosity {
            stop_at_first: false,
            ..Verbosity::default()
        };
        let actual = compare.check(&query);
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_err_only_reports() -> [Vec<Report>; 1] {
    [vec![
        BadExport {
            from: 9583,
            to: 2914,
            items: vec![],
        },
        BadImport {
            from: 9583,
            to: 2914,
            items: vec![],
        },
    ]]
}

#[test]
fn ok_skip_checks() -> Result<()> {
    let query = query()?;
    for (expected, line) in expected_ok_skip_checks().into_iter().zip(LINES) {
        let mut compare = Compare::with_line_dump(line)?;
        compare.verbosity = Verbosity::minimum_all();
        let actual = compare.check(&query);
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_ok_skip_checks() -> [Vec<Report>; 1] {
    [vec![
        BadExport {
            from: 9583,
            to: 2914,
            items: vec![],
        },
        BadImport {
            from: 9583,
            to: 2914,
            items: vec![],
        },
        UnrecExport {
            from: 2914,
            to: 1239,
            items: vec![
                UnrecordedAsSet("AS-ANY".into()),
                UnrecordedAsSetRoute("AS2914:AS-GLOBAL".into()),
            ],
        },
        UnrecImport {
            from: 2914,
            to: 1239,
            items: vec![UnrecordedAutNum(1239)],
        },
        UnrecExport {
            from: 1239,
            to: 3130,
            items: vec![UnrecordedAutNum(1239)],
        },
        UnrecImport {
            from: 1239,
            to: 3130,
            items: vec![UnrecordedAutNum(3130)],
        },
    ]]
}

const DB_FILE: &str = "1239|3130|-1
1239|2914|0
2914|9583|-1
2914|4096|-1
";

#[test]
fn stats() -> Result<()> {
    let query = query()?;
    let db = as_relationship_db()?;
    for (expected, line) in expected_stats().into_iter().zip(LINES) {
        let map = DashMap::new();
        let mut compare = Compare::with_line_dump(line)?;
        compare.as_stats(&query, &db, &map);
        let actual = HashMap::from_iter(map.into_iter());
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_stats() -> [HashMap<u32, RouteStats<u64>>; 1] {
    [
        hashmap! {3130=> RouteStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 0, import_unrec: 1, export_unrec: 0, import_meh: 0, export_meh: 0, import_err: 0, export_err: 0, skip_regex_tilde: 0, skip_regex_with_set: 0, skip_community: 0, unrec_import_empty: 0, unrec_export_empty: 0, unrec_filter_set: 0, unrec_as_routes: 0, unrec_route_set: 0, unrec_as_set: 0, unrec_as_set_route: 0, unrec_some_as_set_route: 0, unrec_aut_num: 1, unrec_peering_set: 0, spec_uphill: 0, spec_uphill_tier1: 0, spec_tier1_pair: 0, spec_import_peer_oifps: 0, spec_import_customer_oifps: 0, spec_export_customers: 0, spec_import_from_neighbor: 0, spec_as_is_origin_but_no_route: 0, spec_as_set_contains_origin_but_no_route: 0, err_filter: 0, err_filter_as_num: 0, err_filter_as_set: 0, err_filter_prefixes: 0, err_filter_route_set: 0, err_remote_as_num: 0, err_remote_as_set: 0, err_except_peering_right: 0, err_peering: 0, err_regex: 0, rpsl_as_name: 0, rpsl_filter: 0, rpsl_regex: 0, rpsl_unknown_filter: 0, recursion: 0 }, 1239=> RouteStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 0, import_unrec: 1, export_unrec: 1, import_meh: 0, export_meh: 0, import_err: 0, export_err: 0, skip_regex_tilde: 0, skip_regex_with_set: 0, skip_community: 0, unrec_import_empty: 0, unrec_export_empty: 0, unrec_filter_set: 0, unrec_as_routes: 0, unrec_route_set: 0, unrec_as_set: 0, unrec_as_set_route: 0, unrec_some_as_set_route: 0, unrec_aut_num: 2, unrec_peering_set: 0, spec_uphill: 0, spec_uphill_tier1: 0, spec_tier1_pair: 0, spec_import_peer_oifps: 0, spec_import_customer_oifps: 0, spec_export_customers: 0, spec_import_from_neighbor: 0, spec_as_is_origin_but_no_route: 0, spec_as_set_contains_origin_but_no_route: 0, err_filter: 0, err_filter_as_num: 0, err_filter_as_set: 0, err_filter_prefixes: 0, err_filter_route_set: 0, err_remote_as_num: 0, err_remote_as_set: 0, err_except_peering_right: 0, err_peering: 0, err_regex: 0, rpsl_as_name: 0, rpsl_filter: 0, rpsl_regex: 0, rpsl_unknown_filter: 0, recursion: 0 }, 9583=> RouteStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 0, import_unrec: 0, export_unrec: 0, import_meh: 0, export_meh: 1, import_err: 0, export_err: 0, skip_regex_tilde: 0, skip_regex_with_set: 0, skip_community: 0, unrec_import_empty: 0, unrec_export_empty: 0, unrec_filter_set: 0, unrec_as_routes: 0, unrec_route_set: 0, unrec_as_set: 0, unrec_as_set_route: 0, unrec_some_as_set_route: 0, unrec_aut_num: 0, unrec_peering_set: 0, spec_uphill: 1, spec_uphill_tier1: 0, spec_tier1_pair: 0, spec_import_peer_oifps: 0, spec_import_customer_oifps: 0, spec_export_customers: 0, spec_import_from_neighbor: 0, spec_as_is_origin_but_no_route: 0, spec_as_set_contains_origin_but_no_route: 0, err_filter: 0, err_filter_as_num: 0, err_filter_as_set: 0, err_filter_prefixes: 0, err_filter_route_set: 0, err_remote_as_num: 0, err_remote_as_set: 0, err_except_peering_right: 0, err_peering: 0, err_regex: 0, rpsl_as_name: 0, rpsl_filter: 0, rpsl_regex: 0, rpsl_unknown_filter: 0, recursion: 0 }, 2914=> RouteStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 0, import_unrec: 0, export_unrec: 1, import_meh: 1, export_meh: 0, import_err: 0, export_err: 0, skip_regex_tilde: 0, skip_regex_with_set: 0, skip_community: 0, unrec_import_empty: 0, unrec_export_empty: 0, unrec_filter_set: 0, unrec_as_routes: 0, unrec_route_set: 0, unrec_as_set: 1, unrec_as_set_route: 0, unrec_some_as_set_route: 0, unrec_aut_num: 0, unrec_peering_set: 0, spec_uphill: 1, spec_uphill_tier1: 0, spec_tier1_pair: 0, spec_import_peer_oifps: 0, spec_import_customer_oifps: 0, spec_export_customers: 0, spec_import_from_neighbor: 0, spec_as_is_origin_but_no_route: 0, spec_as_set_contains_origin_but_no_route: 0, err_filter: 0, err_filter_as_num: 0, err_filter_as_set: 0, err_filter_prefixes: 0, err_filter_route_set: 0, err_remote_as_num: 0, err_remote_as_set: 0, err_except_peering_right: 0, err_peering: 0, err_regex: 0, rpsl_as_name: 0, rpsl_filter: 0, rpsl_regex: 0, rpsl_unknown_filter: 0, recursion: 0 }},
    ]
}

pub fn ir() -> Result<Ir> {
    Ok(serde_json::from_str(IR)?)
}

pub fn query() -> Result<QueryIr> {
    Ok(QueryIr::from_ir(ir()?))
}

pub fn as_relationship_db() -> Result<AsRelDb> {
    AsRelDb::from_lines(DB_FILE.lines())
}

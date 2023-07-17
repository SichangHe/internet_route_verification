use std::collections::HashMap;

use dashmap::DashMap;
use maplit::hashmap;
use parse::*;

use crate::{Report::*, ReportItem::*, SkipReason::*, *};

use super::*;

const DUMP: &str = r#"{"aut_nums":{"2914":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":50472}}}}],"mp_filter":{"AsSet":["AS-CHAOS","NoOp"]}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL","NoOp"]}}]},"ipv6":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL-v6","NoOp"]}}]}}},"9583":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}},"actions":{"pref":"20"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4637}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}}]}}}},"as_sets":{},"route_sets":{},"peering_sets":{},"filter_sets":{},"as_routes":{}}"#;
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
        compare.verbosity = Verbosity {
            stop_at_first: false,
            show_skips: true,
            show_success: true,
            ..Verbosity::default()
        };
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
        NeutralExport {
            from: 2914,
            to: 1239,
            items: vec![
                Skip(AsSetUnrecorded("AS-ANY".into())),
                Skip(AsSetRouteUnrecorded("AS2914:AS-GLOBAL".into())),
            ],
        },
        NeutralImport {
            from: 2914,
            to: 1239,
            items: vec![Skip(AutNumUnrecorded(1239))],
        },
        NeutralExport {
            from: 1239,
            to: 3130,
            items: vec![Skip(AutNumUnrecorded(1239))],
        },
        NeutralImport {
            from: 1239,
            to: 3130,
            items: vec![Skip(AutNumUnrecorded(3130))],
        },
    ]]
}

#[test]
fn stats() -> Result<()> {
    let query = query()?;
    for (expected, line) in expected_stats().into_iter().zip(LINES) {
        let map = DashMap::new();
        let mut compare = Compare::with_line_dump(line)?;
        compare.as_stats(&query, &map);
        let actual = HashMap::from_iter(map.into_iter());
        assert_eq!(expected, actual);
    }
    Ok(())
}

fn expected_stats() -> [HashMap<u64, AsStats>; 1] {
    [
        hashmap! {3130=> AsStats { import_ok: 0, export_ok: 0, import_skip: 1, export_skip: 0, import_err: 0, export_err: 0 }, 1239=> AsStats { import_ok: 0, export_ok: 0, import_skip: 1, export_skip: 1, import_err: 0, export_err: 0 }, 9583=> AsStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 0, import_err: 0, export_err: 1 }, 2914=> AsStats { import_ok: 0, export_ok: 0, import_skip: 0, export_skip: 1, import_err: 1, export_err: 0 }},
    ]
}

fn query() -> Result<QueryDump> {
    let dump: Dump = serde_json::from_str(DUMP)?;
    Ok(QueryDump::from_dump(dump))
}

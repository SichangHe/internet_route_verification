use crate::{
    bgp::{report::MatchProblem::*, Report::*, ReportItem::*, *},
    parse::*,
};

use super::*;

const DUMP: &str = r#"{"aut_nums":{"2914":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":50472}}}}],"mp_filter":{"AsSet":["AS-CHAOS","NoOp"]}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL","NoOp"]}}]},"ipv6":{"unicast":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS-ANY"}}}}],"mp_filter":{"AsSet":["AS2914:AS-GLOBAL-v6","NoOp"]}}]}}},"9583":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}},"actions":{"pref":"20"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4637}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":701}}}}],"mp_filter":{"AsNum":[9583,"NoOp"]}}]}}}},"as_sets":{},"route_sets":{},"peering_sets":{},"filter_sets":{},"as_routes":{}}"#;
const LINES: [&str;1] = [
    "TABLE_DUMP2|1687212000|B|147.28.7.1|3130|1.6.165.0/24|3130 1239 2914 9583|IGP|147.28.7.1|0|0|1239:321 1239:1000 1239:1010|NAG||"
];

#[test]
fn selected_checks() -> Result<()> {
    let dump: Dump = serde_json::from_str(DUMP)?;
    let query = QueryDump::from_dump(dump);

    for (expected_report, line) in reports().into_iter().zip(LINES) {
        let compare = Compare::with_line_dump(line)?;
        let actual_report = compare.check(&query);
        assert_eq!(expected_report, actual_report);
    }
    Ok(())
}

fn reports() -> [Vec<Report>; 1] {
    [vec![
        Bad(vec![NoMatch(NoExportRuleSingle(9583))]),
        Bad(vec![NoMatch(NoExportRule(9583, 2914))]),
        Bad(vec![NoMatch(NoImportRule(2914, 9583))]),
    ]]
}

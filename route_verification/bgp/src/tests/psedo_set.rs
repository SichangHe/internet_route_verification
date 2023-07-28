use super::*;

use {AsPathEntry::Seq, Report::*, ReportItem::*, SkipReason::*, SpecialCase::*};

const DUMP: &str = r#"{"aut_nums":{"45891":{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Set":"AS45891:AS-CUSTOMERS"}}}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":139609}}}}],"mp_filter":{"AsNum":[45891,"NoOp"]}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":60725}}}}],"mp_filter":{"AsNum":[45891,"NoOp"]}}]}}}},"as_sets":{},"route_sets":{},"peering_sets":{},"filter_sets":{},"as_routes":{"134525":["103.2.88.0/24"],"45891":[]}}"#;

const DB_FILE: &str = "139609|45891|-1
45891|134525|-1
";

#[test]
fn export_customers() -> Result<()> {
    let query = query()?;
    let verbosity = Verbosity {
        check_customer: true,
        ..Verbosity::minimum_all()
    };
    let cmp = Compare {
        prefix: "103.2.88.0/24".parse()?,
        as_path: vec![Seq(139609), Seq(45891)],
        recursion_limit: 1,
        verbosity,
    };
    let actual = cmp.check(&query);
    assert_eq!(actual, expected_reports_with_customers());
    Ok(())
}

fn expected_reports_with_customers() -> Vec<Report> {
    vec![
        MehExport {
            from: 45891,
            to: 139609,
            items: vec![Special(ExportCustomers)],
        },
        SkipImport {
            from: 45891,
            to: 139609,
            items: vec![Skip(AutNumUnrecorded(139609))],
        },
    ]
}

fn dump() -> Result<Dump> {
    Ok(serde_json::from_str(DUMP)?)
}

fn db() -> Result<AsRelDb> {
    AsRelDb::from_lines(DB_FILE.lines())
}

fn query() -> Result<QueryDump> {
    Ok(QueryDump::from_dump_and_as_relations(dump()?, &db()?))
}

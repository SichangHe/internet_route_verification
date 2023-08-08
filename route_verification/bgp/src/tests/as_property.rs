use super::{cmp::query, *};

use {Report::*, ReportItem::*, SpecialCase::*};

const NUM: u64 = 18106;

const AUT_NUM18106: &str = r#"{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}},"actions":{"pref":"100"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}}]}}}"#;

const DB_FILE: &str = "174|18106|-1
2914|18106|-1
6939|18106|-1
4657|18106|-1
196844|18106|0
";

const LINE:&str="TABLE_DUMP2|1687212000|B|202.73.40.45|18106|5.44.46.0/24|18106 196844|IGP|202.73.40.45|0|0|6777:2000|NAG||";

#[test]
fn as18106_import_only_provider() -> Result<()> {
    let actual = as18106_property()?;
    let expected = AsProperty {
        import_only_provider: true,
        export_only_provider: true,
    };
    assert_eq!(Some(expected), actual);
    Ok(())
}

#[test]
fn report() -> Result<()> {
    let mut query = query()?;
    let db = relationship_db()?;
    let properties = as18106_property()?.context("Property is None")?;
    query.as_properties.insert(NUM, properties);
    let mut compare = Compare::with_line_dump(LINE)?;
    compare.verbosity = Verbosity::minimum_all();
    let actual = compare.check_with_relationship(&query, &db);
    assert_eq!(expected_reports(), actual);
    Ok(())
}

fn expected_reports() -> Vec<Report> {
    vec![
        BadExport {
            from: 196844,
            to: 18106,
            items: vec![],
        },
        MehImport {
            from: 196844,
            to: 18106,
            items: vec![Special(PeerPairWhenOnlyP2CImports)],
        },
    ]
}

fn as18106_property() -> Result<Option<AsProperty>> {
    let aut_num = as18106()?;
    let db = relationship_db()?;
    Ok(AsProperty::maybe_from_aut_num(NUM, &aut_num, &db))
}

fn as18106() -> Result<AutNum> {
    Ok(serde_json::from_str(AUT_NUM18106)?)
}

fn relationship_db() -> Result<AsRelDb> {
    AsRelDb::from_lines(DB_FILE.lines())
}

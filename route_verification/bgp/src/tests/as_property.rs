use super::*;

const AUT_NUM18106: &str = r#"{"body":"","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}},"actions":{"pref":"100"}}],"mp_filter":"Any"},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}},"actions":{"pref":"100"}}],"mp_filter":"Any"}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":6939}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":174}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":4657}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}},{"mp_peerings":[{"mp_peering":{"remote_as":{"Single":{"Num":2914}}}}],"mp_filter":{"Or":{"left":{"AsNum":[18106,"NoOp"]},"right":{"AsSet":["AS18106:AS-TRANSIT","NoOp"]}}}}]}}}"#;

const DB_FILE: &str = "174|18106|-1
2914|18106|-1
6939|18106|-1
4657|18106|-1
";

#[test]
fn as18106_import_only_provider() -> Result<()> {
    let num = 18106;
    let aut_num = as18106()?;
    let db = relationship_db()?;
    let expected = AsProperty {
        import_only_provider: true,
    };
    let actual = AsProperty::maybe_from_aut_num(num, &aut_num, &db);
    assert_eq!(Some(expected), actual);
    Ok(())
}

fn as18106() -> Result<AutNum> {
    Ok(serde_json::from_str(AUT_NUM18106)?)
}

fn relationship_db() -> Result<AsRelDb> {
    AsRelDb::from_lines(DB_FILE.lines())
}

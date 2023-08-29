use super::*;

#[test]
fn pseudo_set_member_w_mntner() {
    let expected: Vec<String> = ["AS1", "AS2", "m#as-foo#MNTR-ME"]
        .into_iter()
        .map(Into::into)
        .collect();
    let actual = gather_members(&RPSLObject {
        class: "as-set".into(),
        name: "as-foo".into(),
        body: "members: AS1, AS2
mbrs-by-ref: MNTR-ME
"
        .into(),
    });
    assert_eq!(actual, expected);
}

#[test]
fn set_ref_w_mntner() {
    let expected = vec![
        AsOrRouteSet {
            name: "m#as-foo".into(),
            body: "".into(),
            members: vec!["m#as-foo#MNTR-ME".into(), "m#as-foo#MNTR-OTHER".into()],
        },
        AsOrRouteSet {
            name: "m#as-foo#MNTR-ME".into(),
            body: "".into(),
            members: vec!["AS3".into()],
        },
        AsOrRouteSet {
            name: "m#as-foo#MNTR-OTHER".into(),
            body: "".into(),
            members: vec!["AS4".into()],
        },
    ];
    let mut set = BTreeMap::new();
    gather_ref(
        &RPSLObject {
            class: "aut-num".into(),
            name: "AS3".into(),
            body: "member-of: as-foo
mnt-by: MNTR-ME
"
            .into(),
        },
        &mut set,
    );
    gather_ref(
        &RPSLObject {
            class: "aut-num".into(),
            name: "AS4".into(),
            body: "member-of: as-foo
mnt-by: MNTR-OTHER
"
            .into(),
        },
        &mut set,
    );
    let actual = conclude_set(set);
    assert_eq!(actual, expected);
}

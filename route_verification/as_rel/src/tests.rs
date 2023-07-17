use super::*;

const GOOD_LINES: [&str; 2] = ["323|56|-1", "509|9205|0"];
const BAD_LINES: [&str; 3] = ["323|56", "323|56|", "# source:"];

#[test]
fn parse_lines() -> Result<()> {
    for (expected, line) in expected_as_rels().into_iter().zip(GOOD_LINES) {
        let actual = try_parse_as_rel(line)?;
        assert_eq!(expected, actual);
    }
    for line in BAD_LINES {
        let actual = try_parse_as_rel(line);
        assert!(actual.is_err());
    }
    Ok(())
}

fn expected_as_rels() -> [((u64, u64), Relationship); 2] {
    [((323, 56), P2C), ((509, 9205), P2P)]
}

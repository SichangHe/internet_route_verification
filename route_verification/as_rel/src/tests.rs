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

const FILE_LINES: &str = "141760|63927|0
# input clique: 174 209 286 701 1239 1299 2828 2914 3257 3320 3356 3491 5511 6453 6461 6762 6830 7018 12956
# IXP ASes: 1200 4635 5507 6695 7606 8714 9355 9439 9560 9722 9989 11670 15645 17819 18398 21371 24029 24115 24990 35054 40633 42476 43100 47886 48850 50384 55818 57463
# source:topology|BGP|20230701|ripe|rrc00
203188|12741|0
63927|58430|0
1239|4657|-1
2914|4657|-1
";

#[test]
fn parse_mini_file() -> Result<()> {
    let db = AsRelDb::from_lines(FILE_LINES.lines())?;
    assert_eq!(db.get(203188, 12741), Some(P2P));
    assert_eq!(db.get(58430, 63927), Some(P2P));
    assert_eq!(db.get(1239, 4657), Some(P2C));
    assert_eq!(db.get(4657, 2914), Some(C2P));
    Ok(())
}

//! This is originally copied from
//! <https://github.com/cunha/measurements/blob/9a14123b4c9d47297fa4c284ff8dd0834ba73936/bgp/bgpmap/src/lib.rs>.
use crate::bgp::map::{parse_bgpdump_table_dump_v2, AsPathEntry};

use super::*;

#[test]
fn parse_table_dump() -> Result<()> {
    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let (pfx, aspath, vp, communities) = parse_bgpdump_table_dump_v2(line)?;
    let origin = aspath.last().unwrap();
    assert_eq!(pfx.to_string(), "6.132.0.0/14");
    assert_eq!(
        aspath,
        vec![
            AsPathEntry::Seq(34224),
            AsPathEntry::Seq(6939),
            AsPathEntry::Seq(8003)
        ]
    );
    assert_eq!(*origin, AsPathEntry::Seq(8003));
    assert_eq!(vp.asn, 34224);
    assert_eq!(vp.ip.to_string(), "94.156.252.18");
    assert_eq!(communities, vec!["34224:333", "34224:334", "34224:2040"]);

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 {6939} 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let (_pfx, aspath, _vp, _) = parse_bgpdump_table_dump_v2(line)?;
    assert_eq!(
        aspath,
        vec![
            AsPathEntry::Seq(34224),
            AsPathEntry::Set(vec![6939]),
            AsPathEntry::Seq(8003)
        ]
    );

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 {6939,6940} 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let (_pfx, aspath, _vp, _) = parse_bgpdump_table_dump_v2(line)?;
    assert_eq!(
        aspath,
        vec![
            AsPathEntry::Seq(34224),
            AsPathEntry::Set(vec![6939, 6940]),
            AsPathEntry::Seq(8003)
        ]
    );

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 {6939,6940} {8003,8004}|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let (_pfx, aspath, _vp, _) = parse_bgpdump_table_dump_v2(line)?;
    assert_eq!(
        aspath,
        vec![
            AsPathEntry::Seq(34224),
            AsPathEntry::Set(vec![6939, 6940]),
            AsPathEntry::Set(vec![8003, 8004])
        ]
    );

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/34|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "bad-prefix");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "bad-prefix");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939 |IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "as-path-entry-no-match");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 69xx39 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "as-path-entry-no-match");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939 800e|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "as-path-entry-no-match");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 6939e 8000|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "as-path-entry-no-match");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|34224|6.132.0.0/14|34224 {6939, 6940} 8000|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "as-path-entry-no-match");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252.18|3xx4224|6.132.0.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "bad-vp-asn");

    let line = "TABLE_DUMP2|1619481601|B|94.156.252|34224|6.132.0.0/14|34224 6939 8003|IGP|94.156.252.18|0|0|34224:333 34224:334 34224:2040|NAG|||";
    let res = parse_bgpdump_table_dump_v2(line).unwrap_err();
    assert_eq!(res.to_string(), "bad-vp-ip");

    Ok(())
}

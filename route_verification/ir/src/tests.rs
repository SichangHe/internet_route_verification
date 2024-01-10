use super::*;

#[test]
fn ipv6_addr_prefix_range() -> Result<()> {
    let prefix: AddrPfxRange = "2a06:4800:1000::/36".parse()?;
    assert_eq!(
        prefix,
        AddrPfxRange {
            address_prefix: "2a06:4800:1000::/36".parse()?,
            range_operator: RangeOperator::NoOp
        }
    );
    Ok(())
}

#[test]
fn merge_as_routes() -> Result<()> {
    let actual = Ir {
        aut_nums: BTreeMap::new(),
        as_sets: BTreeMap::new(),
        route_sets: BTreeMap::new(),
        peering_sets: BTreeMap::new(),
        filter_sets: BTreeMap::new(),
        as_routes: [
            (
                1,
                vec![
                    "10.1.1.0/24".parse()?,
                    "10.1.2.0/24".parse()?,
                    "10.1.3.0/24".parse()?,
                ],
            ),
            (2, vec!["10.2.1.0/24".parse()?, "10.2.2.0/24".parse()?]),
            (3, vec!["10.3.1.0/24".parse()?]),
        ]
        .into(),
    }
    .merge(Ir {
        aut_nums: BTreeMap::new(),
        as_sets: BTreeMap::new(),
        route_sets: BTreeMap::new(),
        peering_sets: BTreeMap::new(),
        filter_sets: BTreeMap::new(),
        as_routes: [
            (
                1,
                vec!["192.168.1.0/24".parse()?, "192.168.2.0/24".parse()?],
            ),
            (
                2,
                vec![
                    "192.169.1.0/24".parse()?,
                    "192.169.2.0/24".parse()?,
                    "192.169.3.0/24".parse()?,
                ],
            ),
        ]
        .into(),
    });

    let expected = Ir {
        aut_nums: BTreeMap::new(),
        as_sets: BTreeMap::new(),
        route_sets: BTreeMap::new(),
        peering_sets: BTreeMap::new(),
        filter_sets: BTreeMap::new(),
        as_routes: [
            (
                1,
                vec![
                    "10.1.1.0/24".parse()?,
                    "10.1.2.0/24".parse()?,
                    "10.1.3.0/24".parse()?,
                    "192.168.1.0/24".parse()?,
                    "192.168.2.0/24".parse()?,
                ],
            ),
            (
                2,
                vec![
                    "10.2.1.0/24".parse()?,
                    "10.2.2.0/24".parse()?,
                    "192.169.1.0/24".parse()?,
                    "192.169.2.0/24".parse()?,
                    "192.169.3.0/24".parse()?,
                ],
            ),
            (3, vec!["10.3.1.0/24".parse()?]),
        ]
        .into(),
    };

    assert_eq!(expected, actual);

    Ok(())
}

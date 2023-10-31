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

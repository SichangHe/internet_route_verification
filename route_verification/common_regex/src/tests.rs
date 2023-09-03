use super::{set::*, *};

#[test]
fn all_names() {
    for name in [
        OBJECT_NAME,
        ASN,
        ANY,
        PEERAS,
        AS_SET_BASE,
        AS_SET_BASE_OR_ASN,
        AS_SET,
        ROUTE_SET_BASE,
        ROUTE_SET_BASE_OR_ASN,
        ROUTE_SET,
        FILTER_SET_BASE,
        FILTER_SET_BASE_OR_ASN,
        FILTER_SET,
        PEERING_SET_BASE,
        PEERING_SET_BASE_OR_ASN,
        PEERING_SET,
    ] {
        assert!(!regex!(name).is_match(""));
    }
}

#[test]
fn range_operator() {
    for op in ["^-", "^+", "^32", "^20-24"] {
        assert!(regex!(RANGE_OPERATOR).is_match(op));
    }
}

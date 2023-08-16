//! > A set's name is an rpsl word with the following restrictions: All
//! > as-set names start with prefix "as-".  All route-set names start with
//! > prefix "rs-".  All rtr-set names start with prefix "rtrs-".  All
//! > filter-set names start with prefix "fltr-".  All peering-set names
//! > start with prefix "prng-".  For example, as-foo is a valid as-set
//! > name.
//!
//! > Set names can also be hierarchical.  A hierarchical set name is a
//! > sequence of set names and AS numbers separated by colons ":".  At
//! > least one component of such a name must be an actual set name (i.e.
//! > start with one of the prefixes above).  All the set name components
//! > of an hierarchical name has to be of the same type.  For example, the
//! > following names are valid: AS1:AS-CUSTOMERS, AS1:RS-EXPORT:AS2, RS-
//! > EXCEPTIONS:RS-BOGUS.
//!
//! <https://www.rfc-editor.org/rfc/rfc2622#section-5>.

use super::*;

/// RPSL object name, without restriction on first letter.
///
/// > Many objects in RPSL have a name.  An `<object-name>` is made up of
/// > letters, digits, the character underscore "_", and the character
/// > hyphen "-"; the first character of a name must be a letter, and
/// > the last character of a name must be a letter or a digit.
pub const OBJECT_NAME: &str = r"[A-Za-z0-9_\-]*[A-Za-z0-9]";

/// AS number.
pub const ASN: &str = "as[0-9]+";

macro_rules! set_of {
    ($base:expr, $or_name:ident, $set_name:ident, $doc:expr) => {
        pub const $or_name: &str = formatcp!("(?:{})|(?:{})", $base, ASN);

        #[doc = $doc]
        pub const $set_name: &str = formatcp!("(?:{}:)*{}(?::{})", $or_name, $base, $or_name);
    };
}

/// > The keyword ANY matches all routes.
pub const ANY: &str = "any";

/// > The keyword PeerAS can be used instead of the AS number of the peer
/// > AS.  PeerAS is particularly useful when the peering is specified
/// > using an AS expression.
pub const PEERAS: &str = "peeras";

/// Base AS Set name, including `peeras`.
pub const AS_SET_BASE: &str = formatcp!("(?:as-{})|(?:{})", OBJECT_NAME, PEERAS);

set_of!(
    AS_SET_BASE,
    AS_SET_BASE_OR_ASN,
    AS_SET,
    r#"> The as-set attribute defines the name of the set.  It is an RPSL name that starts with "as-"."#
);

/// Base Route Set name.
pub const ROUTE_SET_BASE: &str = formatcp!("rs-{}", OBJECT_NAME);

set_of!(
    ROUTE_SET_BASE,
    ROUTE_SET_BASE_OR_ASN,
    ROUTE_SET,
    r#"> The route-set attribute defines the name of the set.  It is an RPSL name that starts with "rs-"."#
);

/// Base Filter Set name.
pub const FILTER_SET_BASE: &str = formatcp!("fltr-{}", OBJECT_NAME);

set_of!(
    FILTER_SET_BASE,
    FILTER_SET_BASE_OR_ASN,
    FILTER_SET,
    r#"> The filter-set attribute defines the name of the filter.  It is an RPSL name that starts with "fltr-"."#
);

/// Base Peering Set name.
pub const PEERING_SET_BASE: &str = formatcp!("prng-{}", OBJECT_NAME);

set_of!(
    PEERING_SET_BASE,
    PEERING_SET_BASE_OR_ASN,
    PEERING_SET,
    r#"> The peering-set attribute defines the name of the set.  It is an RPSL name that starts with "prng-". "#
);

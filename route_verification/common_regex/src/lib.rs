use const_format::formatcp;

/// Object name, without restriction on first letter.
/// > Many objects in RPSL have a name.  An <object-name> is made up of
/// > letters, digits, the character underscore "_", and the character
/// > hyphen "-"; the first character of a name must be a letter, and
/// > the last character of a name must be a letter or a digit.
pub const OBJECT_NAME: &str = r"[A-Za-z0-9_\-]*[A-Za-z0-9]";

/// AS number.
pub const ASN: &str = "AS[0-9]+";

/// Base AS set name, including `PeerAS`.
pub const AS_SET_BASE: &str = formatcp!("(?:AS-{OBJECT_NAME})|(?:PeerAS)");

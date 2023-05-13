use crate::parse::lex::parse_aut_num_name;

pub use super::*;

#[test]
fn parse_name() {
    assert_eq!(parse_aut_num_name("AS2340").unwrap(), 2340);
    assert_eq!(parse_aut_num_name("AS2340 ").unwrap(), 2340);
    assert!(parse_aut_num_name("AS-2340").is_err());
    assert!(parse_aut_num_name("jfwoe").is_err());
}

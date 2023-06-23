use std::{convert::identity, str::FromStr};

use anyhow::{bail, Context, Ok};
use ipnet::IpNet;
use lazy_regex::regex_captures;
use serde::{Deserialize, Serialize};

/// An address prefix `IpNet` followed by an optional range operator `RangeOperator`.
/// <https://www.rfc-editor.org/rfc/rfc2622#page-5>.
#[derive(Clone, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AddrPfxRange {
    pub address_prefix: IpNet,
    pub range_operator: RangeOperator,
}

impl std::fmt::Debug for AddrPfxRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("AddrPfxRange");
        r.field("address_prefix", &self.address_prefix);
        if self.range_operator != RangeOperator::NoOp {
            r.field("range_operator", &self.range_operator);
        }
        r.finish()
    }
}

impl FromStr for AddrPfxRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, prefix, operator) = get_address_prefix_range_fields(s).context(format!(
            "{s} does not have valid address prefix range structure"
        ))?;
        let address_prefix = prefix
            .parse()
            .context(format!("parsing {prefix} as address prefix"))?;
        let range_operator = operator
            .parse()
            .context(format!("parsing {operator} as range operator"))?;
        Ok(Self {
            address_prefix,
            range_operator,
        })
    }
}

impl AddrPfxRange {
    pub fn contains(&self, other: &IpNet) -> bool {
        address_prefix_contains(&self.address_prefix, self.range_operator, other)
    }
}

pub fn address_prefix_contains(
    address_prefix: &IpNet,
    range_operator: RangeOperator,
    other: &IpNet,
) -> bool {
    match range_operator {
        RangeOperator::NoOp => address_prefix == other,
        RangeOperator::Plus => address_prefix.contains(other),
        RangeOperator::Minus => address_prefix.contains(other) && address_prefix != other,
        RangeOperator::Num(n) => other.prefix_len() == n && address_prefix.contains(other),
        RangeOperator::Range(n, m) => {
            (n..=m).contains(&other.prefix_len()) && address_prefix.contains(other)
        }
    }
}

pub fn get_address_prefix_range_fields(s: &str) -> Option<(&str, &str, &str)> {
    regex_captures!(r"^([[[:xdigit:]]\.:/]+)(\^[-+\d]+)?$", s)
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RangeOperator {
    NoOp,
    /// `^-` is the exclusive more specifics operator; it stands for the more specifics of the address prefix excluding the address prefix itself.  For example, 128.9.0.0/16^- contains all the more specifics of 128.9.0.0/16 excluding 128.9.0.0/16.
    Minus,
    /// `^+` is the inclusive more specifics operator; it stands for the more specifics of the address prefix including the address prefix itself.  For example, 5.0.0.0/8^+ contains all the more specifics of 5.0.0.0/8 including 5.0.0.0/8.
    Plus,
    /// `^n` where `n` is an integer, stands for all the length n specifics of the address prefix.  For example, 30.0.0.0/8^16 contains all the more specifics of 30.0.0.0/8 which are of length 16 such as 30.9.0.0/16.
    Num(u8),
    /// `^n-m` where `n` and `m` are integers, stands for all the length n to length m specifics of the address prefix.  For example, 30.0.0.0/8^24-32 contains all the more specifics of 30.0.0.0/8 which are of length 24 to 32 such as 30.9.9.96/28.
    Range(u8, u8),
}

impl std::fmt::Display for RangeOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RangeOperator::*;
        match self {
            NoOp => write!(f, ""),
            Minus => write!(f, "^-"),
            Plus => write!(f, "^+"),
            Num(n) => write!(f, "^{n}"),
            Range(n, m) => write!(f, "^{n}-{m}"),
        }
    }
}

impl FromStr for RangeOperator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = match s {
            "" => Some(Self::NoOp),
            "^-" => Some(Self::Minus),
            "^+" => Some(Self::Plus),
            _ => None,
        } {
            return Ok(r);
        }

        if let Some((_, num)) = get_range_operator_num(s) {
            let n = num
                .parse()
                .context(format!("parsing {s} as range operator"))?;
            Ok(Self::Num(n))
        } else if let Some((_, num, num1)) = get_range_operator_range(s) {
            let n = num
                .parse()
                .context(format!("parsing {s} as range operator"))?;
            let m = num1
                .parse()
                .context(format!("parsing {s} as range operator"))?;
            if n > m {
                bail!("trivial range operator {s}")
            }
            Ok(Self::Range(n, m))
        } else {
            bail!("{s} is not a valid range operator")
        }
    }
}

pub fn get_range_operator_num(s: &str) -> Option<(&str, &str)> {
    regex_captures!(r"\^(\d{1,2})", s)
}

pub fn get_range_operator_range(s: &str) -> Option<(&str, &str, &str)> {
    regex_captures!(r"\^(\d{1,2})-(\d{1,2})", s)
}

/// `ips` must be sorted.
/// For `ips` shorter than 16, do linear search.
/// Starting from the index of the closest element in `ips`, search right and
/// left for address prefix that, combined with `range_operator`,
/// contains `ip`.
/// Stop searching either end when the index do not point to `ip`'s siblings.
pub fn match_ips(ip: &IpNet, ips: &[IpNet], range_operator: RangeOperator) -> bool {
    if ips.len() < 16 {
        // Linear search if `ips` is small.
        return ips
            .iter()
            .any(|value| address_prefix_contains(value, range_operator, ip));
    }
    let mut left = ips.binary_search(ip).map_or_else(identity, identity);
    let mut right = left;
    // Check center.
    if let Some(value) = ips.get(right) {
        if address_prefix_contains(value, range_operator, ip) {
            return true;
        }
    }
    while right < ips.len() - 1 || left > 0 {
        // Check right.
        if right < ips.len() - 1 {
            right += 1;
            if ip.is_sibling(&ips[right]) {
                if address_prefix_contains(&ips[right], range_operator, ip) {
                    return true;
                }
            } else {
                right = ips.len() - 1;
            }
        }
        // Check left
        if left > 0 {
            left -= 1;
            if ip.is_sibling(&ips[left]) {
                if address_prefix_contains(&ips[left], range_operator, ip) {
                    return true;
                }
            } else {
                left = 0;
            }
        }
    }
    false
}

use std::ops;

use Report::*;

pub enum Report {
    Skip(String),
    Good,
    NoMatch(String),
}

impl ops::BitAnd for Report {
    type Output = Report;

    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Skip(reason) => match other {
                Skip(more) => Report::Skip(reason + "\n" + &more),
                Good => Report::Skip(reason),
                NoMatch(_) => other,
            },
            Good | NoMatch(_) => other,
        }
    }
}

impl ops::BitAnd<Option<Report>> for Report {
    type Output = Report;

    fn bitand(self, other: Option<Report>) -> Self::Output {
        match other {
            other @ Some(_) => self & other,
            None => self,
        }
    }
}

pub fn or(we: Option<Report>, they: Option<Report>) -> Option<Report> {
    match we {
        Some(we) => Some(we & they),
        None => they,
    }
}

pub fn or_known(we: Option<Report>, they: Report) -> Report {
    match we {
        Some(we) => we & they,
        None => they,
    }
}

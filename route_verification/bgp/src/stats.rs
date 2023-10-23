use std::ops::Add;

use as_rel::Relationship;
use dashmap::DashMap;

use super::*;

use Report::*;

mod as_;
mod as_pair;
mod route;
mod up_down_hill;

pub use as_::AsStats;
pub use as_pair::AsPairStats;
pub use route::RouteStats;
pub use up_down_hill::UpDownHillStats;

impl Compare {
    pub fn as_stats(&mut self, query: &QueryIr, db: &AsRelDb, map: &DashMap<u64, AsStats>) {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check_with_relationship(query, db);
        for report in reports {
            as_::one(map, report);
        }
    }

    pub fn up_down_hill_stats(&mut self, query: &QueryIr, db: &AsRelDb) -> UpDownHillStats {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check(query);
        let mut result = UpDownHillStats::default();
        for report in reports.iter() {
            up_down_hill::one(&mut result, report, db);
        }
        result
    }

    pub fn as_pair_stats(
        &mut self,
        query: &QueryIr,
        db: &AsRelDb,
        map: &DashMap<(u64, u64), AsPairStats>,
    ) {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check_with_relationship(query, db);
        for report in reports {
            as_pair::one(db, map, report);
        }
    }

    pub fn route_stats(&mut self, query: &QueryIr, db: &AsRelDb) -> RouteStats {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check_with_relationship(query, db);
        let mut stats = RouteStats::default();
        for report in reports {
            route::one(&mut stats, report);
        }
        stats
    }
}

use std::ops::Add;

use as_rel::Relationship;
use dashmap::DashMap;

use super::*;

use Report::*;

mod as_;
mod as_pair;
mod up_down_hill;

pub use as_::AsStats;
pub use as_pair::AsPairStats;
pub use up_down_hill::UpDownHillStats;

impl Compare {
    pub fn as_stats(&mut self, dump: &QueryDump, map: &DashMap<u64, AsStats>) {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check(dump);
        for report in reports {
            as_::one(map, report);
        }
    }

    pub fn up_down_hill_stats(&mut self, dump: &QueryDump, db: &AsRelDb) -> UpDownHillStats {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check(dump);
        let mut result = UpDownHillStats::default();
        for report in reports.iter() {
            up_down_hill::one(&mut result, report, db);
        }
        result
    }

    pub fn as_pair_stats(
        &mut self,
        dump: &QueryDump,
        db: &AsRelDb,
        map: &DashMap<(u64, u64), AsPairStats>,
    ) {
        self.verbosity = Verbosity::minimum_all();
        let reports = self.check(dump);
        for report in reports {
            as_pair::one(db, map, report);
        }
    }
}

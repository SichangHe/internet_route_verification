use super::report::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Verbosity {
    ErrOnly,
    Brief,
    ShowSkips,
    Detailed,
}

pub trait VerbosityReport {
    fn verbosity(&self) -> Verbosity;

    fn skip_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> SkipReason,
    {
        if self.verbosity() >= Verbosity::ShowSkips {
            skip_any_report(reason())
        } else {
            empty_skip_any_report()
        }
    }

    fn no_match_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.verbosity() >= Verbosity::Detailed {
            no_match_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn bad_rpsl_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> RpslError,
    {
        if self.verbosity() >= Verbosity::Detailed {
            bad_rpsl_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn skip_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> SkipReason,
    {
        if self.verbosity() >= Verbosity::ShowSkips {
            skip_all_report(reason())
        } else {
            empty_skip_all_report()
        }
    }

    fn no_match_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.verbosity() >= Verbosity::Detailed {
            no_match_all_report(reason())
        } else {
            failed_all_report()
        }
    }

    fn bad_rpsl_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> RpslError,
    {
        if self.verbosity() >= Verbosity::Detailed {
            bad_rpsl_all_report(reason())
        } else {
            failed_all_report()
        }
    }
}

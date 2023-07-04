use super::report::*;

/// Verbosity level.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Verbosity {
    /// Only report errors. Stops at the first error.
    ErrOnly,
    /// Report errors and success.
    Brief,
    /// Report errors, success and skips.
    ShowSkips,
    /// Report error information for each RPSL policy entry.
    PerEntry,
    /// All errors.
    Detailed,
}

pub trait VerbosityReport {
    fn get_verbosity(&self) -> Verbosity;

    fn success_report<F>(&self, reason: F) -> Option<Report>
    where
        F: Fn() -> SuccessType,
    {
        if self.get_verbosity() >= Verbosity::Brief {
            Some(Report::success(reason()))
        } else {
            None
        }
    }

    fn skips_report(&self, skips: Vec<ReportItem>) -> Option<Report> {
        if self.get_verbosity() >= Verbosity::ShowSkips {
            Some(Report::Neutral(skips))
        } else {
            None
        }
    }

    fn skip_report<F>(&self, reason: F) -> Option<Report>
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity() >= Verbosity::ShowSkips {
            Some(Report::skip(reason()))
        } else {
            None
        }
    }

    fn skip_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity() >= Verbosity::ShowSkips {
            skip_any_report(reason())
        } else {
            empty_skip_any_report()
        }
    }

    fn no_match_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.get_verbosity() >= Verbosity::Detailed {
            no_match_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn bad_rpsl_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> RpslError,
    {
        if self.get_verbosity() >= Verbosity::Detailed {
            bad_rpsl_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn skip_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity() >= Verbosity::ShowSkips {
            skip_all_report(reason())
        } else {
            empty_skip_all_report()
        }
    }

    fn no_match_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.get_verbosity() >= Verbosity::Detailed {
            no_match_all_report(reason())
        } else {
            failed_all_report()
        }
    }

    fn bad_rpsl_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> RpslError,
    {
        if self.get_verbosity() >= Verbosity::Detailed {
            bad_rpsl_all_report(reason())
        } else {
            failed_all_report()
        }
    }
}

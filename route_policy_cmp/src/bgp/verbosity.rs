use super::report::*;

/// Verbosity level.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Verbosity {
    /// Stop checking the AS path at the first [`Report`].
    pub stop_at_first: bool,
    /// Report skips.
    pub show_skips: bool,
    /// Report success.
    pub show_success: bool,
    /// Report error information for each RPSL policy entry.
    pub per_entry_err: bool,
    /// All errors.
    pub all_err: bool,
}

impl std::fmt::Debug for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_set();
        let Verbosity {
            stop_at_first: stop_at_error,
            show_skips,
            show_success,
            per_entry_err,
            all_err,
        } = self;
        for (is_true, tag) in [
            (stop_at_error, "stop_at_error"),
            (show_skips, "show_skips"),
            (show_success, "show_success"),
            (per_entry_err, "per_entry_err"),
            (all_err, "all_err"),
        ] {
            if *is_true {
                result.entry(&tag);
            }
        }
        result.finish()
    }
}

impl Verbosity {
    pub fn new(
        stop_at_error: bool,
        show_skips: bool,
        show_success: bool,
        per_entry_err: bool,
        all_err: bool,
    ) -> Self {
        Self {
            stop_at_first: stop_at_error,
            show_skips,
            show_success,
            per_entry_err,
            all_err,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Self {
            stop_at_first: true,
            show_skips: false,
            show_success: false,
            per_entry_err: false,
            all_err: false,
        }
    }
}

pub trait VerbosityReport {
    fn get_verbosity(&self) -> Verbosity;

    fn success_report<F>(&self, reason: F) -> Option<Report>
    where
        F: Fn() -> SuccessType,
    {
        if self.get_verbosity().show_success {
            Some(Report::success(reason()))
        } else {
            None
        }
    }

    fn skips_report(&self, skips: Vec<ReportItem>) -> Option<Report> {
        if self.get_verbosity().show_skips {
            Some(Report::Neutral(skips))
        } else {
            None
        }
    }

    fn skip_report<F>(&self, reason: F) -> Option<Report>
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity().show_skips {
            Some(Report::skip(reason()))
        } else {
            None
        }
    }

    fn skip_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity().show_skips {
            skip_any_report(reason())
        } else {
            empty_skip_any_report()
        }
    }

    fn skip_any_reports<F, I>(&self, reasons: F) -> AnyReport
    where
        F: Fn() -> I,
        I: IntoIterator<Item = SkipReason>,
    {
        if self.get_verbosity().show_skips {
            skip_any_reports(reasons())
        } else {
            empty_skip_any_report()
        }
    }

    fn no_match_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.get_verbosity().all_err {
            no_match_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn bad_rpsl_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> RpslError,
    {
        if self.get_verbosity().all_err {
            bad_rpsl_any_report(reason())
        } else {
            failed_any_report()
        }
    }

    fn skip_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> SkipReason,
    {
        if self.get_verbosity().show_skips {
            skip_all_report(reason())
        } else {
            empty_skip_all_report()
        }
    }

    fn no_match_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> MatchProblem,
    {
        if self.get_verbosity().all_err {
            no_match_all_report(reason())
        } else {
            failed_all_report()
        }
    }

    fn bad_rpsl_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> RpslError,
    {
        if self.get_verbosity().all_err {
            bad_rpsl_all_report(reason())
        } else {
            failed_all_report()
        }
    }
}

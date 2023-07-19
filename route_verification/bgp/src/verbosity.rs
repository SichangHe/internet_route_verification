use super::*;

#[allow(unused)] // For the doc.
use Report::*;

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
    /// Record [`AsPathPairWithSet`], [`SetImport`], [`SetExport`].
    pub report_set: bool,
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
            report_set,
        } = self;
        for (is_true, tag) in [
            (stop_at_error, "stop_at_error"),
            (show_skips, "show_skips"),
            (show_success, "show_success"),
            (per_entry_err, "per_entry_err"),
            (all_err, "all_err"),
            (report_set, "report_set"),
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
        stop_at_first: bool,
        show_skips: bool,
        show_success: bool,
        per_entry_err: bool,
        all_err: bool,
        report_set: bool,
    ) -> Self {
        Self {
            stop_at_first,
            show_skips,
            show_success,
            per_entry_err,
            all_err,
            report_set,
        }
    }

    /// Report all errors, skips, and success but not the details.
    pub const fn minimum_all() -> Self {
        Self {
            stop_at_first: false,
            show_skips: true,
            show_success: true,
            ..Self::const_default()
        }
    }

    pub const fn const_default() -> Self {
        Self {
            stop_at_first: true,
            show_skips: false,
            show_success: false,
            per_entry_err: false,
            all_err: false,
            report_set: false,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::const_default()
    }
}

pub trait VerbosityReport {
    fn get_verbosity(&self) -> Verbosity;

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

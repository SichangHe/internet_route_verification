use crate::lex::action;

use super::*;

/// <https://www.rfc-editor.org/rfc/rfc2622#section-6.1.1>
pub type Actions = BTreeMap<String, Action>;

pub fn parse_actions(actions: action::Actions) -> Actions {
    actions
}

/// <https://www.rfc-editor.org/rfc/rfc2622#section-7>
/// For now, we do not further parse `<action>`s.
pub type Action = action::Action;

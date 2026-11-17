//! The date poll rule (answers Yes, If needed, No): how many said Yes, "If needed" and No in a window, and which window is suggested.
//! Herkunft: fs-decisions@5cd8806:crates/fs-decisions-simple/src/grid.rs (renamed: `Tally` is `AnswerCount`, `tally` is `count_answers`, `best` is `best_window`, because `Tally` and `tally` already name the result type of this crate).
//!
//! Pattern: Strategy (one pure function per rule) over plain value types; the rule is the decision of the project owner
//! (17 Nov 2026): no weighting of "If needed", the three answers stay apart, the rule stands next to the suggestion.

use std::cmp::Reverse;

/// One answer of one person for one time window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    /// Yes, I can.
    Yes,
    /// Only if it must be ("If needed").
    IfNeeded,
    /// No, I cannot.
    No,
}

/// The answers of one window, counted apart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnswerCount {
    /// How many said Yes.
    pub yes: u32,
    /// How many said "If needed".
    pub if_needed: u32,
    /// How many said No.
    pub no: u32,
}

impl AnswerCount {
    /// The commitments: Yes plus "If needed" (no weighting).
    #[must_use]
    pub fn commitments(self) -> u32 {
        self.yes + self.if_needed
    }
}

/// Count the answers of one window.
#[must_use]
pub fn count_answers(answers: impl IntoIterator<Item = Answer>) -> AnswerCount {
    answers.into_iter().fold(AnswerCount::default(), |mut t, a| {
        match a {
            Answer::Yes => t.yes += 1,
            Answer::IfNeeded => t.if_needed += 1,
            Answer::No => t.no += 1,
        }
        t
    })
}

/// The window to suggest: the one with the most commitments; on a tie the one with fewer "If needed"; on a further tie the
/// earliest (the lowest index). `None` when nobody has committed to any window. A person decides; this only suggests.
#[must_use]
pub fn best_window(windows: &[AnswerCount]) -> Option<usize> {
    windows
        .iter()
        .enumerate()
        .filter(|(_, t)| t.commitments() > 0)
        .min_by_key(|(index, t)| (Reverse(t.commitments()), t.if_needed, *index))
        .map(|(index, _)| index)
}

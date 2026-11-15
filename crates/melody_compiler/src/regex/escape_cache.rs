#![allow(dead_code)]

const RESERVED: &str = r"[](){}*+?|^$.";

/// Report whether the regex backend would prefix this character with a slash.
pub(super) fn needs_escape(character: char) -> bool {
    RESERVED.contains(character)
}

/// Estimate the emitted byte width for one literal character.
pub(super) fn emitted_width(character: char) -> usize {
    character.len_utf8() + usize::from(needs_escape(character))
}

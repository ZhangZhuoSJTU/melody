#![allow(dead_code)]

/// Check the narrow ASCII identifier shape without allocating a `String`.
pub(super) fn is_ascii_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    matches!(
        bytes.next(),
        Some(b'_' | b'a'..=b'z' | b'A'..=b'Z')
    ) && bytes.all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

/// Find the first non-ASCII byte offset, if the string contains one.
pub(super) fn first_non_ascii(value: &str) -> Option<usize> {
    value.bytes().position(|byte| !byte.is_ascii())
}

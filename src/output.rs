pub const MAX_OUTPUT_BYTES: usize = 48 * 1024;

/// Decode command output and cap it before it reaches the UI.
pub fn bounded(bytes: &[u8]) -> String {
    let mut text = String::from_utf8_lossy(bytes).into_owned();
    if text.len() > MAX_OUTPUT_BYTES {
        let mut end = MAX_OUTPUT_BYTES;
        while !text.is_char_boundary(end) { end -= 1; }
        text.truncate(end);
        text.push_str("\n\n[Output truncated by WinForge to keep the interface responsive.]");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_small_output() { assert_eq!(bounded(b"hello"), "hello"); }
    #[test]
    fn backs_off_before_a_multibyte_character() {
        let prefix = "a".repeat(MAX_OUTPUT_BYTES - 1);
        let input = format!("{prefix}界-and-more");
        let result = bounded(input.as_bytes());
        assert!(result.starts_with(&prefix));
        assert!(!result.starts_with(&format!("{prefix}界")));
    }

    #[test]
    fn truncates_without_splitting_utf8() {
        let input = "界".repeat(MAX_OUTPUT_BYTES);
        let result = bounded(input.as_bytes());
        assert!(result.ends_with("[Output truncated by WinForge to keep the interface responsive.]"));
    }
}
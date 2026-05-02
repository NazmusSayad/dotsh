pub fn format(key: &str, value: &str) -> String {
    let escaped = value.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$");
    format!("set -gx {} \"{}\"", key, escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value() {
        assert_eq!(format("KEY", "val"), "set -gx KEY \"val\"");
    }

    #[test]
    fn empty_value() {
        assert_eq!(format("KEY", ""), "set -gx KEY \"\"");
    }

    #[test]
    fn value_with_spaces() {
        assert_eq!(format("KEY", "val with spaces"), "set -gx KEY \"val with spaces\"");
    }

    #[test]
    fn value_with_backslash() {
        assert_eq!(
            format("KEY", r"C:\Users\name"),
            r#"set -gx KEY "C:\\Users\\name""#
        );
    }

    #[test]
    fn value_with_double_quote() {
        assert_eq!(
            format("KEY", r#"He said "hello""#),
            r#"set -gx KEY "He said \"hello\"""#
        );
    }

    #[test]
    fn value_with_dollar() {
        assert_eq!(
            format("KEY", "$HOME/bin"),
            r#"set -gx KEY "\$HOME/bin""#
        );
    }

    #[test]
    fn value_with_single_quote() {
        assert_eq!(
            format("KEY", "it's"),
            "set -gx KEY \"it's\""
        );
    }

    #[test]
    fn value_with_semicolon() {
        assert_eq!(
            format("KEY", "a;b;c"),
            "set -gx KEY \"a;b;c\""
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(
            format("KEY", "a=b"),
            "set -gx KEY \"a=b\""
        );
    }

    #[test]
    fn windows_path() {
        assert_eq!(
            format("CARGO_HOME", r"C:\Users\Sayad\.cargo"),
            r#"set -gx CARGO_HOME "C:\\Users\\Sayad\\.cargo""#
        );
    }

    #[test]
    fn value_with_multiple_special_chars() {
        assert_eq!(
            format("KEY", r#"$HOME\docs"say""#),
            r#"set -gx KEY "\$HOME\\docs\"say\"""#
        );
    }
}

pub fn format(key: &str, value: &str) -> String {
    let escaped = value.replace('\'', "'\\''");
    format!("export {}='{}'", key, escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value() {
        assert_eq!(format("KEY", "val"), "export KEY='val'");
    }

    #[test]
    fn empty_value() {
        assert_eq!(format("KEY", ""), "export KEY=''");
    }

    #[test]
    fn value_with_spaces() {
        assert_eq!(format("KEY", "val with spaces"), "export KEY='val with spaces'");
    }

    #[test]
    fn value_with_single_quote() {
        assert_eq!(format("KEY", "it's"), "export KEY='it'\\''s'");
    }

    #[test]
    fn value_with_multiple_single_quotes() {
        assert_eq!(
            format("KEY", "it's a test's"),
            "export KEY='it'\\''s a test'\\''s'"
        );
    }

    #[test]
    fn value_with_backslash() {
        assert_eq!(format("KEY", r"C:\Users\name"), r"export KEY='C:\Users\name'");
    }

    #[test]
    fn value_with_double_quote() {
        assert_eq!(
            format("KEY", r#"He said "hello""#),
            r#"export KEY='He said "hello"'"#
        );
    }

    #[test]
    fn value_with_semicolon() {
        assert_eq!(
            format("KEY", "a;b;c"),
            "export KEY='a;b;c'"
        );
    }

    #[test]
    fn value_with_dollar() {
        assert_eq!(
            format("KEY", "$HOME/bin"),
            "export KEY='$HOME/bin'"
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(
            format("KEY", "a=b"),
            "export KEY='a=b'"
        );
    }
}

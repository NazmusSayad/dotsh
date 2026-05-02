pub fn format(key: &str, value: &str) -> String {
    format!("set \"{}={}\"", key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value() {
        assert_eq!(format("KEY", "val"), "set \"KEY=val\"");
    }

    #[test]
    fn empty_value() {
        assert_eq!(format("KEY", ""), "set \"KEY=\"");
    }

    #[test]
    fn value_with_spaces() {
        assert_eq!(format("KEY", "val with spaces"), "set \"KEY=val with spaces\"");
    }

    #[test]
    fn value_with_semicolon() {
        assert_eq!(
            format("KEY", "a;b;c"),
            "set \"KEY=a;b;c\""
        );
    }

    #[test]
    fn value_with_double_quote() {
        assert_eq!(
            format("KEY", r#"He said "hello""#),
            "set \"KEY=He said \"hello\"\"",
        );
    }

    #[test]
    fn value_with_backslash() {
        assert_eq!(
            format("KEY", r"C:\Users\name"),
            r#"set "KEY=C:\Users\name""#
        );
    }

    #[test]
    fn value_with_single_quote() {
        assert_eq!(
            format("KEY", "it's"),
            "set \"KEY=it's\""
        );
    }

    #[test]
    fn value_with_dollar() {
        assert_eq!(
            format("KEY", "$HOME/bin"),
            "set \"KEY=$HOME/bin\""
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(
            format("KEY", "a=b"),
            "set \"KEY=a=b\""
        );
    }

    #[test]
    fn windows_path() {
        assert_eq!(
            format("PATH", r"C:\Windows;C:\Users"),
            r#"set "PATH=C:\Windows;C:\Users""#
        );
    }
}

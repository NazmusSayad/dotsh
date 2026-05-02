pub fn format(key: &str, value: &str) -> String {
    let escaped = value.replace('\'', "''");
    format!("$env:{} = '{}'", key, escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value() {
        assert_eq!(format("KEY", "val"), "$env:KEY = 'val'");
    }

    #[test]
    fn empty_value() {
        assert_eq!(format("KEY", ""), "$env:KEY = ''");
    }

    #[test]
    fn value_with_spaces() {
        assert_eq!(format("KEY", "val with spaces"), "$env:KEY = 'val with spaces'");
    }

    #[test]
    fn value_with_single_quote() {
        assert_eq!(format("KEY", "it's"), "$env:KEY = 'it''s'");
    }

    #[test]
    fn value_with_multiple_single_quotes() {
        assert_eq!(
            format("KEY", "it's a test's"),
            "$env:KEY = 'it''s a test''s'"
        );
    }

    #[test]
    fn value_with_backslash() {
        assert_eq!(
            format("KEY", r"C:\Users\name"),
            r"$env:KEY = 'C:\Users\name'"
        );
    }

    #[test]
    fn value_with_double_quote() {
        assert_eq!(
            format("KEY", r#"He said "hello""#),
            r#"$env:KEY = 'He said "hello"'"#
        );
    }

    #[test]
    fn value_with_semicolon() {
        assert_eq!(
            format("KEY", "a;b;c"),
            "$env:KEY = 'a;b;c'"
        );
    }

    #[test]
    fn value_with_dollar() {
        assert_eq!(
            format("KEY", "$HOME/bin"),
            "$env:KEY = '$HOME/bin'"
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(
            format("KEY", "a=b"),
            "$env:KEY = 'a=b'"
        );
    }

    #[test]
    fn windows_path() {
        assert_eq!(
            format("CARGO_HOME", r"C:\Users\Sayad\.cargo"),
            r"$env:CARGO_HOME = 'C:\Users\Sayad\.cargo'"
        );
    }
}

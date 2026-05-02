pub fn parse(input: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        result.push((key.trim().to_string(), value.trim().to_string()));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(parse(""), Vec::<(String, String)>::new());
    }

    #[test]
    fn single_pair() {
        assert_eq!(parse("KEY=val"), vec![("KEY".to_string(), "val".to_string())]);
    }

    #[test]
    fn multiple_pairs() {
        let input = "A=1\nB=2\nC=3";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
                ("C".to_string(), "3".to_string()),
            ]
        );
    }

    #[test]
    fn empty_lines_skipped() {
        let input = "A=1\n\n\nB=2";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn whitespace_only_lines_skipped() {
        let input = "A=1\n   \n\t\nB=2";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn comment_lines_skipped() {
        let input = "A=1\n# this is a comment\nB=2";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn comment_with_leading_whitespace() {
        let input = "A=1\n  # comment\nB=2";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn line_without_equals_skipped() {
        let input = "A=1\nno_equals_here\nB=2";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn key_trimmed() {
        assert_eq!(
            parse("  KEY  =val"),
            vec![("KEY".to_string(), "val".to_string())]
        );
    }

    #[test]
    fn value_trimmed() {
        assert_eq!(
            parse("KEY=  val  "),
            vec![("KEY".to_string(), "val".to_string())]
        );
    }

    #[test]
    fn both_trimmed() {
        assert_eq!(
            parse("  KEY  =  val  "),
            vec![("KEY".to_string(), "val".to_string())]
        );
    }

    #[test]
    fn empty_value() {
        assert_eq!(
            parse("KEY="),
            vec![("KEY".to_string(), "".to_string())]
        );
    }

    #[test]
    fn value_with_equals() {
        assert_eq!(
            parse("KEY=a=b=c"),
            vec![("KEY".to_string(), "a=b=c".to_string())]
        );
    }

    #[test]
    fn value_with_spaces() {
        assert_eq!(
            parse("KEY=val with spaces"),
            vec![("KEY".to_string(), "val with spaces".to_string())]
        );
    }

    #[test]
    fn value_with_backslashes() {
        assert_eq!(
            parse(r"KEY=C:\Users\name"),
            vec![("KEY".to_string(), r"C:\Users\name".to_string())]
        );
    }

    #[test]
    fn value_with_single_quotes() {
        assert_eq!(
            parse("KEY=it's working"),
            vec![("KEY".to_string(), "it's working".to_string())]
        );
    }

    #[test]
    fn value_with_double_quotes() {
        assert_eq!(
            parse(r#"KEY=He said "hello""#),
            vec![("KEY".to_string(), r#"He said "hello""#.to_string())]
        );
    }

    #[test]
    fn value_with_semicolons() {
        assert_eq!(
            parse("KEY=a;b;c"),
            vec![("KEY".to_string(), "a;b;c".to_string())]
        );
    }

    #[test]
    fn value_with_dollar_signs() {
        assert_eq!(
            parse("KEY=$HOME/bin"),
            vec![("KEY".to_string(), "$HOME/bin".to_string())]
        );
    }

    #[test]
    fn key_with_dots() {
        assert_eq!(
            parse("KEY.Name=VALUE"),
            vec![("KEY.Name".to_string(), "VALUE".to_string())]
        );
    }

    #[test]
    fn mixed_valid_and_invalid_lines() {
        let input = "A=1\n\n# comment\nBAD\n  \nB=2\nC=";
        assert_eq!(
            parse(input),
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
                ("C".to_string(), "".to_string()),
            ]
        );
    }

    #[test]
    fn value_with_hash() {
        assert_eq!(
            parse("KEY=val#ue"),
            vec![("KEY".to_string(), "val#ue".to_string())]
        );
    }

    #[test]
    fn value_with_tabs() {
        assert_eq!(
            parse("KEY=val\tue"),
            vec![("KEY".to_string(), "val\tue".to_string())]
        );
    }

    #[test]
    fn key_with_underscores_and_numbers() {
        assert_eq!(
            parse("KEY_1_NAME=val"),
            vec![("KEY_1_NAME".to_string(), "val".to_string())]
        );
    }

    #[test]
    fn multiple_empty_values() {
        assert_eq!(
            parse("A=\nB=\nC="),
            vec![
                ("A".to_string(), "".to_string()),
                ("B".to_string(), "".to_string()),
                ("C".to_string(), "".to_string()),
            ]
        );
    }

    #[test]
    fn windows_path_value() {
        assert_eq!(
            parse(r"PATH=C:\Windows\System32;C:\Users\name"),
            vec![(
                "PATH".to_string(),
                r"C:\Windows\System32;C:\Users\name".to_string()
            )]
        );
    }
}

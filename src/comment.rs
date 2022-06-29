pub fn extract(text: &str) -> Option<String> {
    if let Some(text) = text.strip_prefix("//") {
        Some(text.trim().into())
    } else if text.starts_with("/*") && text.ends_with("*/") {
        let mut lines = text[2..text.len() - 2]
            .trim_matches('*')
            .split('\n')
            .skip_while(|line| line.trim().is_empty())
            .collect::<Vec<_>>();

        lines.splice(
            lines.len()
                - lines
                    .iter()
                    .rev()
                    .take_while(|line| line.trim().is_empty())
                    .count()..,
            [],
        );

        Some(lines.join("\n"))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_a() {
        assert!(extract("/ abc de f/").is_none());
        assert!(extract("/*abc de fg/").is_none());
        assert!(extract("/abc de fg*/").is_none());
    }

    #[test]
    fn inline() {
        assert_eq!(extract("// abc de f").unwrap(), "abc de f");
    }

    #[test]
    fn inline_compact() {
        assert_eq!(extract("//abc de f").unwrap(), "abc de f");
    }

    #[test]
    fn inline_sparse() {
        assert_eq!(extract("//   abc  de f  ").unwrap(), "abc  de f");
    }

    #[test]
    fn singleline() {
        assert_eq!(extract("/* abc de f */").unwrap(), " abc de f ");
    }

    #[test]
    fn singleline_compact() {
        assert_eq!(extract("/*abc de f*/").unwrap(), "abc de f");
    }

    #[test]
    fn singleline_sparse() {
        assert_eq!(extract("/*   abc de  f  */").unwrap(), "   abc de  f  ");
    }

    #[test]
    fn multiline() {
        assert_eq!(
            extract(
                r#"/*

abc de

f gh


*/"#
            )
            .unwrap(),
            r#"abc de

f gh"#
        );
    }

    #[test]
    fn multiline_verbatim() {
        assert_eq!(
            extract(
                r#"/*


 abc de

   f gh

 */"#
            )
            .unwrap(),
            r#" abc de

   f gh"#
        );
    }

    #[test]
    fn multiline_verbatim_l() {
        assert_eq!(
            extract(
                r#"/*
 * abc de
 *   f gh
 */"#
            )
            .unwrap(),
            r#" * abc de
 *   f gh"#
        );
    }

    #[test]
    fn multiline_verbatim_lr() {
        assert_eq!(
            extract(
                r#"/*******
 * abc de *
 *  f gh  *
 *******/"#
            )
            .unwrap(),
            r#" * abc de *
 *  f gh  *"#
        );
    }
}

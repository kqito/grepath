pub mod eslint;
pub mod tsc;

#[cfg(test)]
mod utils {
    use crate::grep::GrepItem;
    use pretty_assertions::assert_eq;

    pub fn assert_paths<'a>(content: Vec<GrepItem>, expected: Vec<&str>) {
        let paths: Vec<&str> = content.iter().map(|item| item.path.as_str()).collect();
        assert_eq!(paths, expected);
    }
}

#[cfg(test)]
mod tests {
    use crate::grep::{grep, params::GrepParamsBuilder, tests::utils::assert_paths};

    #[test]
    fn extract_path_test() {
        let params = GrepParamsBuilder::new()
            .content(Some(
                "hello world /home/username.txt /etc/passwd.txt".to_string(),
            ))
            .build()
            .unwrap();

        assert_paths(grep(&params), vec!["etc/passwd.txt", "home/username.txt"]);
    }

    #[test]
    fn no_match_text() {
        let params = GrepParamsBuilder::new()
            .content(Some("hello world /home/username /etc/passwd".to_string()))
            .build()
            .unwrap();

        assert_paths(grep(&params), vec![]);
    }

    #[test]
    fn extract_path_from_error_message() {
        let params = GrepParamsBuilder::new()
            .content(Some(r#"error: "tsc" exited with code 2"#.to_string()))
            .build()
            .unwrap();

        assert_paths(grep(&params), Vec::<&str>::new());
    }
}

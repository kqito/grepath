#[cfg(test)]
mod tests {
    use crate::grep::finder::{Finder, Location};
    use crate::grep::GrepItem;
    use crate::grep::{grep, params::GrepParamsBuilder};
    use pretty_assertions::assert_eq;
    use std::fmt::Debug;

    pub fn assert_paths<'a>(content: Vec<GrepItem>, expected: Vec<&str>) {
        let paths: Vec<&str> = content.iter().map(|item| item.path.as_str()).collect();
        assert_eq!(paths, expected);
    }

    #[derive(Debug, Clone)]
    struct MockFinder {
        current_dir: String,
        ignore_pattern: Vec<String>,
        files: Vec<String>,
    }

    impl MockFinder {
        fn new(files: Vec<String>) -> Self {
            Self {
                current_dir: ".".to_string(),
                ignore_pattern: vec![],
                files,
            }
        }
    }

    impl Finder for MockFinder {
        fn current_dir(&mut self, current_dir: &str) {
            self.current_dir = current_dir.to_string();
        }

        fn ignore(&mut self, ignore: Vec<String>) {
            self.ignore_pattern = ignore;
        }

        fn find(&mut self) -> Location {
            let paths = self.files.clone();
            Location { paths }
        }
    }

    #[test]
    fn test_grep() {
        let files = vec![
            "test_data/test_file_1.txt".to_string(),
            "test_data/test_file_2.txt".to_string(),
            "test_data/test_file_3.txt".to_string(),
        ];

        let input_content =
            "Error occurred in test_data/test_file_1.txt and test_data/test_file_2.txt:1:1";

        let params = GrepParamsBuilder::new()
            .content(Some(input_content.to_string()))
            .finder(Box::new(MockFinder::new(files)))
            .build()
            .expect("Failed to build GrepParams");

        let grep_items = grep(&params);

        assert_paths(
            grep_items,
            vec!["test_data/test_file_1.txt", "test_data/test_file_2.txt"],
        );
    }

    #[test]
    fn test_grep_start_with_dot_slash() {
        let files = vec![
            "./test_data/test_file_1.txt".to_string(),
            "./test_data/test_file_2.txt".to_string(),
            "./test_data/test_file_3.txt".to_string(),
        ];

        let input_content = "./test_data/test_file_1.txt sample content to test\n./test_data/test_file_2.txt:1:1: sample content to test";

        let params = GrepParamsBuilder::new()
            .content(Some(input_content.to_string()))
            .finder(Box::new(MockFinder::new(files)))
            .build()
            .expect("Failed to build GrepParams");

        let grep_items = grep(&params);

        assert_paths(
            grep_items,
            vec!["./test_data/test_file_1.txt", "./test_data/test_file_2.txt"],
        );
    }

    #[test]
    fn test_grep_without_unique() {
        let files = vec![
            "test_data/test_file_1.txt".to_string(),
            "test_data/test_file_2.txt".to_string(),
            "test_data/test_file_3.txt".to_string(),
        ];

        let input_content = "test_data/test_file_2.txt:1:1 test_data/test_file_2.txt:1:1";

        let params = GrepParamsBuilder::new()
            .unique(Some(false))
            .content(Some(input_content.to_string()))
            .finder(Box::new(MockFinder::new(files)))
            .build()
            .expect("Failed to build GrepParams");

        let grep_items = grep(&params);

        assert_paths(
            grep_items,
            vec!["test_data/test_file_2.txt", "test_data/test_file_2.txt"],
        );
    }

    #[test]
    fn test_grep_with_unique() {
        let files = vec![
            "test_data/test_file_1.txt".to_string(),
            "test_data/test_file_2.txt".to_string(),
            "test_data/test_file_3.txt".to_string(),
        ];

        let input_content = "test_data/test_file_2.txt:1:1 test_data/test_file_2.txt:1:1";

        let params = GrepParamsBuilder::new()
            .unique(Some(true))
            .content(Some(input_content.to_string()))
            .finder(Box::new(MockFinder::new(files)))
            .build()
            .expect("Failed to build GrepParams");

        let grep_items = grep(&params);

        assert_paths(grep_items, vec!["test_data/test_file_2.txt"]);
    }
}

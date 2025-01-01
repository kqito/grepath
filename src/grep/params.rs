use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct GrepParams {
    pub debug: bool,
    pub unique: bool,
    pub content: String,
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
}

impl Default for GrepParams {
    fn default() -> Self {
        Self {
            debug: false,
            unique: true,
            validate: true,
            content: "".to_string(),
            current_dir: ".".to_string(),
            ignore_pattern: vec![],
        }
    }
}

#[derive(Default, Debug)]
pub struct GrepParamsBuilder {
    debug: Option<bool>,
    unique: Option<bool>,
    content: Option<String>,
    current_dir: Option<String>,
    ignore_pattern: Vec<String>,
}

impl GrepParamsBuilder {
    pub fn new() -> Self {
        Self {
            debug: None,
            unique: None,
            content: None,
            current_dir: None,
            ignore_pattern: vec![],
        }
    }

    pub fn debug(mut self, debug: Option<bool>) -> Self {
        self.debug = debug;
        self
    }

    pub fn unique(mut self, unique: Option<bool>) -> Self {
        self.unique = unique;
        self
    }

    pub fn content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }

    pub fn read_file_content(mut self, file: &str) -> Result<Self, Error> {
        let content = std::fs::read_to_string(file)?;
        self.content = Some(content);

        Ok(self)
    }

    pub fn current_dir(mut self, current_dir: Option<String>) -> Self {
        self.current_dir = current_dir;
        self
    }

    pub fn ignore(mut self, ignore: Vec<String>) -> Self {
        self.ignore_pattern = [ignore, self.ignore_pattern].concat();
        self
    }

    pub fn build(self) -> Result<GrepParams, Error> {
        let default_ignore_pattern: Vec<String> = vec![
            // Version control directories
            ".git".to_string(),
            ".svn".to_string(),
            ".hg".to_string(),
            ".bzr".to_string(),
            // Node.js
            "node_modules".to_string(),
            // Build output
            "dist".to_string(),
            "build".to_string(),
            "out".to_string(),
            // Documentation
            ".yardoc".to_string(),
            ".bundle".to_string(),
            "docs".to_string(),
            "documentation".to_string(),
            // OS specific files
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            // Compiled files
            "*.pyc".to_string(),
            "*.pyo".to_string(),
            // Executables/Binaries
            "*.exe".to_string(),
            "*.dll".to_string(),
            "*.so".to_string(),
            "*.dylib".to_string(),
            // Logs and coverage
            "coverage".to_string(),
            "*.log".to_string(),
            ".coverage".to_string(),
            "*.lcov".to_string(),
        ];

        match self.content {
            Some(content) => Ok(GrepParams {
                debug: self.debug.unwrap_or(false),
                unique: self.unique.unwrap_or(true),
                content,
                current_dir: self.current_dir.unwrap_or(".".to_string()),
                ignore_pattern: match self.ignore_pattern.is_empty() {
                    true => default_ignore_pattern,
                    false => self.ignore_pattern,
                },
            }),
            None => Err(anyhow!("Missing content to search for")),
        }
    }
}

#[cfg(test)]
mod params {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default_params() {
        let params = GrepParams::default();

        assert_eq!(params.unique, true);
        assert_eq!(params.content, "".to_string());
    }

    #[test]
    fn build_params() {
        let params = GrepParamsBuilder::new()
            .unique(Some(false))
            .content(Some("hello world".to_string()))
            .build()
            .unwrap();

        assert_eq!(params.unique, false);
        assert_eq!(params.content, "hello world".to_string());
    }

    #[test]
    fn build_params_without_unique() {
        let params = GrepParamsBuilder::new()
            .content(Some("hello world".to_string()))
            .build()
            .unwrap();

        assert_eq!(params.unique, true);
        assert_eq!(params.content, "hello world".to_string());
    }

    #[test]
    fn build_params_without_content() {
        let params = GrepParamsBuilder::new().unique(Some(false)).build();

        assert!(params.is_err());
    }
}

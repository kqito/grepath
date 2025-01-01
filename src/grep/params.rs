use crate::grep::finder::{Finder, IOFinder};
use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
pub struct GrepParams {
    pub debug: bool,
    pub unique: bool,
    pub content: String,
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
    pub finder: Box<dyn Finder>,
}

#[derive(Debug, Clone)]
pub struct GrepParamsBuilder {
    debug: Option<bool>,
    unique: Option<bool>,
    content: Option<String>,
    current_dir: Option<String>,
    ignore_pattern: Vec<String>,
    finder: Option<Box<dyn Finder>>,
}

impl GrepParamsBuilder {
    pub fn new() -> Self {
        Self {
            debug: None,
            unique: None,
            content: None,
            current_dir: None,
            ignore_pattern: vec![],
            finder: None,
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

    #[cfg(test)]
    pub fn finder(mut self, finder: Box<dyn Finder>) -> Self {
        self.finder = Some(finder);
        self
    }

    pub fn build(self) -> Result<GrepParams, Error> {
        let default_ignore_pattern: Vec<String> = vec![
            ".git".to_string(),
            ".svn".to_string(),
            ".hg".to_string(),
            ".bzr".to_string(),
            "node_modules".to_string(),
            "dist".to_string(),
            "build".to_string(),
            "out".to_string(),
            ".yardoc".to_string(),
            ".bundle".to_string(),
            "docs".to_string(),
            "documentation".to_string(),
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            "*.pyc".to_string(),
            "*.pyo".to_string(),
            "*.exe".to_string(),
            "*.dll".to_string(),
            "*.so".to_string(),
            "*.dylib".to_string(),
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
                ignore_pattern: if self.ignore_pattern.is_empty() {
                    default_ignore_pattern
                } else {
                    self.ignore_pattern
                },
                finder: self.finder.unwrap_or_else(|| Box::new(IOFinder::new())),
            }),
            None => Err(anyhow!("Missing content to search for")),
        }
    }
}

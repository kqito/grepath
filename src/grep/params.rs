use crate::grep::finder::{Finder, IOFinder};
use anyhow::{anyhow, Error};

#[derive(Debug, Clone, PartialEq)]
pub enum Filetype {
    Directory,
    File,
}

#[derive(Debug, Clone)]
pub struct GrepParams {
    pub debug: bool,
    pub content: String,
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
    pub finder: Box<dyn Finder>,
    pub filetype: Vec<Filetype>,
}

#[derive(Debug, Clone)]
pub struct GrepParamsBuilder {
    debug: Option<bool>,
    content: Option<String>,
    current_dir: Option<String>,
    ignore_pattern: Vec<String>,
    finder: Option<Box<dyn Finder>>,
    filetype: Vec<String>,
}

impl GrepParamsBuilder {
    pub fn new() -> Self {
        Self {
            debug: None,
            content: None,
            current_dir: None,
            ignore_pattern: vec![],
            finder: None,
            filetype: vec![],
        }
    }

    pub fn debug(mut self, debug: Option<bool>) -> Self {
        self.debug = debug;
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

    pub fn filetype(mut self, filetype: Vec<String>) -> Self {
        self.filetype = [filetype, self.filetype].concat();
        self
    }

    #[cfg(test)]
    pub fn finder(mut self, finder: Box<dyn Finder>) -> Self {
        self.finder = Some(finder);
        self
    }

    pub fn build(self) -> Result<GrepParams, Error> {
        let default_ignore_pattern: Vec<String> =
            vec!["*node_modules*".to_string(), "*.git/*".to_string()];

        let filetype = match self.filetype.len() {
            // When no filetype is provided, default to file
            0 => vec![Filetype::File],
            _ => self.filetype.iter().fold(vec![], |mut acc, filetype| {
                match filetype.as_str() {
                    "d" | "dir" => acc.push(Filetype::Directory),
                    "f" | "file" => acc.push(Filetype::File),
                    _ => {}
                };
                acc
            }),
        };

        let current_dir = self.current_dir.clone();
        let debug = self.debug.unwrap_or(false);
        let mut default_finder = IOFinder::new();
        default_finder.current_dir(&current_dir.unwrap_or(".".to_string()));
        default_finder.ignore(default_ignore_pattern.clone());
        default_finder.debug(debug);

        let finder = self.finder.unwrap_or_else(|| Box::new(default_finder));

        match self.content {
            Some(content) => Ok(GrepParams {
                debug: self.debug.unwrap_or(false),
                content,
                current_dir: self.current_dir.unwrap_or(".".to_string()),
                ignore_pattern: if self.ignore_pattern.is_empty() {
                    default_ignore_pattern
                } else {
                    self.ignore_pattern
                },
                finder,
                filetype,
            }),
            None => Err(anyhow!("Missing content to search for")),
        }
    }
}

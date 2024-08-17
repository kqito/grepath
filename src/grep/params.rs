use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct GrepParams {
    pub lines: bool,
    pub unique: bool,
    pub content: String,
}

impl Default for GrepParams {
    fn default() -> Self {
        Self {
            lines: false,
            unique: true,
            content: "".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct GrepParamsBuilder {
    lines: Option<bool>,
    unique: Option<bool>,
    content: Option<String>,
}

impl GrepParamsBuilder {
    pub fn new() -> Self {
        Self {
            lines: None,
            unique: None,
            content: None,
        }
    }

    pub fn lines(mut self, lines: Option<bool>) -> Self {
        self.lines = lines;
        self
    }

    pub fn unique(mut self, unique: Option<bool>) -> Self {
        self.unique = unique;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn read_file_content(mut self, file: &str) -> Result<Self, Error> {
        let content = std::fs::read_to_string(file)?;
        self.content = Some(content);

        Ok(self)
    }

    pub fn build(self) -> Result<GrepParams, Error> {
        match self.content {
            Some(content) => Ok(GrepParams {
                lines: self.lines.unwrap_or(false),
                unique: self.unique.unwrap_or(true),
                content,
            }),
            None => Err(anyhow!("content is required")),
        }
    }
}

#[cfg(test)]
mod params {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn default_params() {
        let params = GrepParams::default();

        assert_eq!(params.lines, false);
        assert_eq!(params.unique, true);
        assert_eq!(params.content, "".to_string());
    }

    #[test]
    fn build_params() {
        let params = GrepParamsBuilder::new()
            .lines(Some(true))
            .unique(Some(false))
            .content("hello world".to_string())
            .build()
            .unwrap();

        assert_eq!(params.lines, true);
        assert_eq!(params.unique, false);
        assert_eq!(params.content, "hello world".to_string());
    }

    #[test]
    fn build_params_without_lines() {
        let params = GrepParamsBuilder::new()
            .unique(Some(false))
            .content("hello world".to_string())
            .build()
            .unwrap();

        assert_eq!(params.lines, false);
        assert_eq!(params.unique, false);
        assert_eq!(params.content, "hello world".to_string());
    }

    #[test]
    fn build_params_without_unique() {
        let params = GrepParamsBuilder::new()
            .lines(Some(true))
            .content("hello world".to_string())
            .build()
            .unwrap();

        assert_eq!(params.lines, true);
        assert_eq!(params.unique, true);
        assert_eq!(params.content, "hello world".to_string());
    }

    #[test]
    fn build_params_without_content() {
        let params = GrepParamsBuilder::new()
            .lines(Some(true))
            .unique(Some(false))
            .build();

        assert!(params.is_err());
    }
}

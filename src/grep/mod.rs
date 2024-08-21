pub mod params;
mod tests;

use std::path::Path;

use params::GrepParams;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum GrepItemType {
    RelativePath,
    AbsolutePath,
}

#[derive(Debug, PartialEq)]
pub struct GrepItem {
    pub path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub item_type: GrepItemType,
}

/// Extract path in string message with regex
pub fn grep(params: &GrepParams) -> Vec<GrepItem> {
    let mut items: Vec<GrepItem> = Vec::new();
    let regex = Regex::new(r"\b[\w./-]+?\.\w+\b").unwrap();

    // Iterate over all matches in the message
    for cap in regex.captures_iter(&params.content) {
        let path = cap[0].to_string();
        let parts: Vec<&str> = path.split(':').collect();
        let line: Option<usize> = match parts.get(1) {
            Some(line) => match line.parse::<usize>() {
                Ok(line) => Some(line),
                Err(_) => None,
            },
            None => None,
        };

        let column: Option<usize> = match parts.get(2) {
            Some(column) => match column.parse::<usize>() {
                Ok(column) => Some(column),
                Err(_) => None,
            },
            None => None,
        };

        let item_type = match path.starts_with('/') {
            true => GrepItemType::AbsolutePath,
            false => GrepItemType::RelativePath,
        };

        if params.validate {
            if !Path::new(&path).exists() {
                continue;
            }
        }

        items.push(GrepItem {
            path,
            line,
            column,
            item_type,
        });
    }

    // debup by item.path
    items.sort_by(|a, b| {
        a.path
            .partial_cmp(&b.path)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if params.unique {
        items.dedup_by(|a, b| a.path == b.path);
    }

    items
}

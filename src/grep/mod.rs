mod finder;
pub mod params;
mod tests;

use std::path::Path;

use finder::Finder;
use params::GrepParams;

use crate::output::{pretty_print, Status};

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
    let mut finder = Finder::new();
    finder.current_dir(&params.current_dir);
    finder.ignore(params.ignore_pattern.clone());
    let find_list = finder.find();

    if params.debug {
        pretty_print(&format!("Finder: {:#?}", &find_list), Status::Info);
        pretty_print(
            &format!("Finder Regex: {:#?}", &find_list.as_regex()),
            Status::Info,
        );
    }

    // Iterate over all matches in the message
    for cap in find_list.as_regex().captures_iter(&params.content) {
        let matched = cap[0].to_string();
        let parts: Vec<&str> = matched.split(':').collect();

        let path = parts.get(0).unwrap().to_string();
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

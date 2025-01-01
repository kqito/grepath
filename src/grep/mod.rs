mod finder;
pub mod params;
mod tests;

use crate::output::{pretty_print, Status};
use params::{Filetype, GrepParams};
use rayon::prelude::*;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

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
    pub filetype: Filetype,
}

/// Extract path in string message with regex
pub fn grep(params: &GrepParams) -> Vec<GrepItem> {
    let finder = Arc::new(RwLock::new(params.finder.clone()));
    let mut finder_lock = finder.write().unwrap();
    finder_lock.current_dir(&params.current_dir);
    finder_lock.ignore(params.ignore_pattern.clone());
    let find_list = finder_lock.find();
    drop(finder_lock);

    if params.debug {
        pretty_print(&format!("Content: {:#?}", &params.content), Status::Info);
        pretty_print(&format!("Finder: {:#?}", &find_list), Status::Info);
    }

    let items: Vec<_> = find_list
        .resources
        .par_iter()
        .filter_map(|r| {
            // Improve performance by checking if it matches without using regular expressions
            if !params.content.contains(&r.path) {
                return None;
            }

            if params.debug {
                pretty_print(&format!("Matched: {}", &r.path), Status::Info);
            }

            let numbers = match r.as_regex().find(&params.content) {
                Some(numbers) => numbers,
                None => return None,
            };
            let line = match numbers.as_str().parse::<usize>() {
                Ok(line) => Some(line),
                Err(_) => None,
            };
            let column = match numbers.as_str().parse::<usize>() {
                Ok(column) => Some(column),
                Err(_) => None,
            };
            let item_type = match &r.path.starts_with('/') {
                true => GrepItemType::AbsolutePath,
                false => GrepItemType::RelativePath,
            };

            Some(GrepItem {
                path: r.path.clone(),
                line,
                column,
                item_type,
                filetype: r.filetype.clone(),
            })
        })
        .collect();

    let mut unique_items: Vec<GrepItem> = items.into_iter().collect();
    // dedup by item.path
    unique_items.sort_by(|a, b| {
        a.path
            .partial_cmp(&b.path)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    unique_items.dedup_by(|a, b| a.path == b.path);

    // filetypeでのフィルタリング
    unique_items.retain(|item| match item.filetype {
        Filetype::File => params.filetype.contains(&Filetype::File),
        Filetype::Directory => params.filetype.contains(&Filetype::Directory),
    });

    unique_items
}

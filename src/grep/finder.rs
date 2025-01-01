use regex::Regex;
use std::fmt::Debug;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct IOFinder {
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
}

#[derive(Debug)]
pub struct Location {
    pub paths: Vec<String>,
}

pub trait Finder: FinderClone + Debug {
    fn current_dir(&mut self, current_dir: &str);
    fn ignore(&mut self, ignore: Vec<String>);
    fn find(&mut self) -> Location;
}

pub trait FinderClone {
    fn clone_box(&self) -> Box<dyn Finder>;
}

impl<T> FinderClone for T
where
    T: 'static + Finder + Clone,
{
    fn clone_box(&self) -> Box<dyn Finder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Finder> {
    fn clone(&self) -> Box<dyn Finder> {
        self.clone_box()
    }
}

impl IOFinder {
    pub fn new() -> Self {
        Self {
            current_dir: ".".to_string(),
            ignore_pattern: vec![],
        }
    }

    fn is_ignored(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        for pattern in &self.ignore_pattern {
            if path_str.contains(pattern) {
                return true;
            }
        }

        false
    }
}

impl Finder for IOFinder {
    fn current_dir(&mut self, current_dir: &str) {
        self.current_dir = current_dir.to_string();
    }

    fn ignore(&mut self, ignore: Vec<String>) {
        self.ignore_pattern = ignore;
    }

    fn find(&mut self) -> Location {
        let mut paths: Vec<String> = Vec::new();
        let walker = WalkDir::new(&self.current_dir).into_iter();

        for entry in walker.filter_entry(|e| !self.is_ignored(e)) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    paths.push(entry.path().display().to_string());
                }
            }
        }

        Location { paths }
    }
}

impl Location {
    pub fn as_regex(&self) -> Regex {
        let matches: Vec<String> = self
            .paths
            .iter()
            .map(|path| {
                let path = path.replace("/", r"\/");
                format!(r"{}(:\d+:\d+)?", path)
            })
            .collect();

        let pattern = matches.join("|");
        Regex::new(&pattern).unwrap()
    }
}

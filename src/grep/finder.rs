use regex::Regex;
use std::fmt::Debug;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct IOFinder {
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Directory,
    File,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
    pub resource_type: ResourceType,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub resources: Vec<Resource>,
}

pub trait Finder: FinderClone + Debug {
    fn current_dir(&mut self, current_dir: &str);
    fn ignore(&mut self, ignore: Vec<String>);
    fn find(&mut self) -> Stats;
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

    fn find(&mut self) -> Stats {
        let mut resources: Vec<Resource> = Vec::new();
        let walker = WalkDir::new(&self.current_dir).into_iter();

        for entry in walker.filter_entry(|e| !self.is_ignored(e)) {
            if let Ok(entry) = entry {
                resources.push(Resource {
                    path: entry.path().to_str().unwrap().to_string(),
                    resource_type: if entry.file_type().is_dir() {
                        ResourceType::Directory
                    } else {
                        ResourceType::File
                    },
                });
            }
        }

        Stats { resources }
    }
}

impl Stats {
    pub fn as_regex(&self) -> Regex {
        let mut matches: Vec<String> = self
            .resources
            .iter()
            .map(|r| {
                let path = r.path.replace("/", r"\/");
                match r.resource_type {
                    ResourceType::Directory => format!(r"{}", path),
                    // Support for line and column numbers
                    ResourceType::File => format!(r"{}(:\d+:\d+)?", path),
                }
            })
            .collect();
        matches.sort_by(|a, b| b.len().cmp(&a.len()));

        let pattern = matches.join("|");
        Regex::new(&pattern).unwrap()
    }
}

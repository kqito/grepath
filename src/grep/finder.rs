use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;
use walkdir::{DirEntry, WalkDir};

use super::params::Filetype;

#[derive(Debug, Clone)]
pub struct IOFinder {
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
    pub filetype: Filetype,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub resources: Vec<Resource>,
}

pub trait Finder: FinderClone + Debug + Sync + Send {
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
            let segment = path_str.split("/").collect::<Vec<&str>>();
            if segment.iter().any(|s| s == pattern) {
                return true;
            }
        }

        false
    }

    fn find_iterative(&self, start_dir: &str, resources: &mut Vec<Resource>) {
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start_dir.to_string());

        while let Some(dir) = queue.pop_front() {
            let walker = WalkDir::new(&dir).into_iter();

            for entry in walker.filter_entry(|e| !self.is_ignored(e)) {
                match entry {
                    Ok(entry) => {
                        let path_str = entry.path().to_str().unwrap();
                        let path = if path_str.starts_with("./") {
                            path_str[2..].to_string()
                        } else {
                            path_str.to_string()
                        };

                        let filetype = if entry.file_type().is_dir() {
                            Filetype::Directory
                        } else {
                            Filetype::File
                        };

                        if resources.iter().any(|r| r.path == path) {
                            continue;
                        }

                        resources.push(Resource {
                            path: path.clone(),
                            filetype,
                        });

                        if entry.file_type().is_dir() {
                            queue.push_back(path);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accessing {}: {}", dir, e);
                    }
                }
            }
        }
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
        self.find_iterative(&self.current_dir, &mut resources);
        Stats { resources }
    }
}

impl Resource {
    pub fn as_regex(&self) -> Regex {
        match self.filetype {
            Filetype::Directory => {
                let path = self.path.replace("/", r"\/");
                Regex::new(&format!(r"{}", path)).unwrap()
            }
            // Support for line and column numbers
            Filetype::File => {
                let path = self.path.replace("/", r"\/");
                Regex::new(&format!(r"{}(:\d+:\d+)?", path)).unwrap()
            }
        }
    }
}

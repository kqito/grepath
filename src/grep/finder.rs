use crate::output::pretty_print;
use crate::output::Status;
use ignore::gitignore::Gitignore;
use ignore::Match;
use rayon::prelude::*;
use regex::Regex;
use std::fmt::Debug;
use std::path::{Component, Path};
use walkdir::{DirEntry, WalkDir};
use wildmatch::WildMatch;
use wildmatch::WildMatchPattern;

use super::params::Filetype;

fn extract_relative_path(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    let mut components = path.components().peekable();
    let mut result = String::new();

    while let Some(component) = components.next() {
        match component {
            Component::ParentDir => {
                if let Some(last_slash) = result.rfind('/') {
                    result.truncate(last_slash);
                } else {
                    result.clear();
                }
            }
            Component::CurDir => {}
            _ => {
                if !result.is_empty() {
                    result.push('/');
                }
                result.push_str(component.as_os_str().to_str().unwrap_or(""));
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[derive(Clone)]
pub struct IOFinder {
    pub current_dir: String,
    pub ignore_pattern: Vec<WildMatchPattern<'*', '?'>>,
    pub debug: bool,
    gitignore: Option<Gitignore>,
}

impl Debug for IOFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOFinder")
            .field("current_dir", &self.current_dir)
            .field(
                "ignore_pattern",
                &self
                    .ignore_pattern
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>(),
            )
            .field("debug", &self.debug)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
    pub matcher: String,
    pub filetype: Filetype,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub resources: Vec<Resource>,
}

pub trait Finder: FinderClone + Debug + Sync + Send {
    fn current_dir(&mut self, current_dir: &str);
    fn ignore(&mut self, ignore: Vec<String>);
    fn debug(&mut self, debug: bool);
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
            debug: false,
            gitignore: None,
        }
    }

    fn is_ignored(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return false,
        };

        if let Some(gitignore) = &self.gitignore {
            match gitignore.matched(path, path.is_dir()) {
                Match::Ignore(_) => {
                    if self.debug {
                        pretty_print(
                            &format!("Skip: {:#?} by .gitignore", path_str),
                            Status::Info,
                        );
                    }
                    return true;
                }
                _ => {}
            }
        }

        self.ignore_pattern.iter().any(|pattern| {
            if pattern.matches(path_str) {
                if self.debug {
                    pretty_print(
                        &format!("Skip: {:#?} by {:#?}", path_str, pattern.to_string()),
                        Status::Info,
                    );
                }
                true
            } else {
                false
            }
        })
    }

    fn find_parallel(&self, start_dir: &str) -> Vec<Resource> {
        WalkDir::new(start_dir)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e))
            .filter_map(|entry| entry.ok())
            .par_bridge()
            .map(|entry| {
                let path = entry.path().to_str().unwrap().to_string();
                let filetype = if entry.file_type().is_dir() {
                    Filetype::Directory
                } else {
                    Filetype::File
                };
                let matcher = extract_relative_path(&path).unwrap_or(path.clone());
                Resource {
                    path,
                    matcher,
                    filetype,
                }
            })
            .collect()
    }
}

impl Finder for IOFinder {
    fn current_dir(&mut self, current_dir: &str) {
        self.current_dir = current_dir.to_string();
    }

    fn ignore(&mut self, ignore: Vec<String>) {
        self.ignore_pattern = ignore
            .into_iter()
            .map(|i| WildMatch::new(&i))
            .collect::<Vec<_>>();
    }

    fn find(&mut self) -> Stats {
        let gitignore_path = format!("{}/.gitignore", &self.current_dir);
        let (gitignore, err) = Gitignore::new(gitignore_path);

        if err.is_none() {
            self.gitignore = Some(gitignore);
        }

        let resources = self.find_parallel(&self.current_dir);

        Stats { resources }
    }

    fn debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}

impl Resource {
    pub fn as_regex(&self) -> Regex {
        match self.filetype {
            Filetype::Directory => {
                let path = self.path.replace('/', r"\/");
                Regex::new(&format!(r"{}", path)).unwrap()
            }
            Filetype::File => {
                let path = self.path.replace('/', r"\/");
                Regex::new(&format!(r"{}(:\d+:\d+)?", path)).unwrap()
            }
        }
    }
}

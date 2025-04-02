use crate::output::pretty_print;
use crate::output::Status;
use ignore::gitignore::Gitignore;
use ignore::Match;
use regex::Regex;
use std::fmt::Debug;
use std::{collections::VecDeque, path::PathBuf};
use walkdir::{DirEntry, WalkDir};
use wildmatch::WildMatch;
use wildmatch::WildMatchPattern;

use super::params::Filetype;

use std::path::{Component, Path};

fn extract_relative_path(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    let mut components = path.components().peekable();
    let mut result = Vec::new();

    while let Some(component) = components.next() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            _ => {
                result.push(component.as_os_str());
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(
            Path::new(&result.iter().collect::<PathBuf>())
                .to_string_lossy()
                .to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct IOFinder {
    pub current_dir: String,
    pub ignore_pattern: Vec<WildMatchPattern<'*', '?'>>,
    pub debug: bool,
    gitignore: Option<Gitignore>,
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

        if let Some(gitignore) = &self.gitignore {
            match gitignore.matched(path, path.is_dir()) {
                Match::Ignore(_) => {
                    if self.debug {
                        pretty_print(
                            &format!("Skip: {:#?} by .gitignore", path.to_str()),
                            Status::Info,
                        );
                    }
                    return true;
                }
                _ => {}
            }
        }

        let path_str = match path.to_str() {
            Some(path) => path,
            None => return false,
        };

        for pattern in &self.ignore_pattern {
            if pattern.matches(path_str) {
                if self.debug {
                    pretty_print(
                        &format!("Skip: {:#?} by {:#?}", path, pattern.to_string()),
                        Status::Info,
                    );
                }
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
                        let path = entry.path().to_str().unwrap();

                        let filetype = if entry.file_type().is_dir() {
                            Filetype::Directory
                        } else {
                            Filetype::File
                        };

                        if resources.iter().any(|r| r.path == path) {
                            continue;
                        }

                        resources.push(Resource {
                            path: path.to_string(),
                            matcher: extract_relative_path(path).unwrap_or(path.to_string()),
                            filetype,
                        });

                        if entry.file_type().is_dir() {
                            queue.push_back(path.to_string());
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
        self.ignore_pattern = ignore
            .into_iter()
            .map(|i| WildMatch::new(&i))
            .collect::<Vec<_>>();
    }

    fn find(&mut self) -> Stats {
        let mut resources: Vec<Resource> = Vec::new();
        let gitignore_path = format!("{}/.gitignore", &self.current_dir);
        let (gitignore, err) = Gitignore::new(gitignore_path);

        if err.is_none() {
            self.gitignore = Some(gitignore);
        }
        self.find_iterative(&self.current_dir, &mut resources);

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
                let path = self.path.replace("/", r"\/");
                Regex::new(&format!(r"{}", path)).unwrap()
            }
            Filetype::File => {
                let path = self.path.replace("/", r"\/");
                Regex::new(&format!(r"{}(:\d+:\d+)?", path)).unwrap()
            }
        }
    }
}

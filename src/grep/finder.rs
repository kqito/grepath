use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Finder {
    pub current_dir: String,
    pub ignore_pattern: Vec<String>,
}

#[derive(Debug)]
pub struct FindResult {
    paths: Vec<String>,
}

impl Finder {
    pub fn new() -> Self {
        Self {
            current_dir: ".".to_string(),
            ignore_pattern: vec![],
        }
    }

    pub fn current_dir(&mut self, current_dir: &str) {
        self.current_dir = current_dir.to_string();
    }

    pub fn ignore(&mut self, ignore: Vec<String>) {
        self.ignore_pattern = ignore;
    }

    pub fn find(&mut self) -> FindResult {
        let mut paths: Vec<String> = Vec::new();
        let walker = WalkDir::new(&self.current_dir).into_iter();

        for entry in walker.filter_entry(|e| !self.is_ignored(e)) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    paths.push(entry.path().display().to_string());
                }
            }
        }

        FindResult { paths }
    }

    fn is_ignored(&mut self, entry: &walkdir::DirEntry) -> bool {
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        for pattern in self.ignore_pattern.clone() {
            if path_str.contains(&pattern) {
                return true;
            }
        }

        false
    }
}

impl FindResult {
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

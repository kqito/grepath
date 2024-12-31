use std::io::{self, Read};

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Args
pub struct Args {
    #[argh(positional)]
    /// file
    pub file: Option<String>,

    #[argh(switch, short = 'd')]
    /// help
    pub debug: Option<bool>,

    #[argh(switch, short = 'u')]
    /// unique
    /// Omit duplicate paths
    pub unique: Option<bool>,

    #[argh(switch)]
    /// no-validate
    /// Skip validation of the file
    pub no_validate: Option<bool>,

    #[argh(option, short = 'c')]
    /// current_dir
    /// Set the current directory
    /// Default: "."
    /// Example: /path/to/dir
    pub current_dir: Option<String>,

    #[argh(option, short = 'i')]
    /// ignore
    /// Ignore pattern
    pub ignore: Vec<String>,
}

pub fn get_input() -> Result<Option<String>> {
    let mut pipeline_input = String::new();
    let is_piped = atty::isnt(atty::Stream::Stdin);

    if is_piped {
        if io::stdin().read_to_string(&mut pipeline_input).is_err() {
            return Err(anyhow::anyhow!("Failed to read from stdin"));
        }
    }

    let content = if pipeline_input == "" {
        Ok(None)
    } else {
        Ok(Some(pipeline_input))
    };

    content
}

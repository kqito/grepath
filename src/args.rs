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

    #[argh(switch, short = 'l')]
    /// lines
    /// Extract only the file paths without line numbers
    /// e.g. src/main.rs
    pub lines: Option<bool>,

    #[argh(switch, short = 'u')]
    /// unique
    /// Omit duplicate paths
    pub unique: Option<bool>,
}

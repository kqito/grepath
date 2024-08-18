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
}

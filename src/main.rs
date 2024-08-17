mod args;
mod grep;
mod output;

use std::io::{self, Read};

use args::Args;
use grep::grep;
use grep::params::GrepParamsBuilder;
use output::{pretty_print, Status};

fn main() {
    let args: Args = argh::from_env();

    let mut pipeline_input = String::new();
    // Read from stdin if it's not a tty
    if atty::isnt(atty::Stream::Stdin) {
        if io::stdin().read_to_string(&mut pipeline_input).is_err() {
            pretty_print("Failed to read from stdin", Status::Error);
            return;
        }
    }

    let mut params_builder = GrepParamsBuilder::new()
        .lines(args.lines)
        .unique(args.unique)
        .content(pipeline_input);

    if let Some(f) = args.file {
        match params_builder.read_file_content(&f) {
            Ok(builder) => params_builder = builder,
            Err(e) => {
                pretty_print(&e.to_string(), Status::Error);
                return;
            }
        }
    }

    let params = match params_builder.build() {
        Ok(params) => params,
        Err(e) => {
            pretty_print(&e.to_string(), Status::Error);
            return;
        }
    };

    if let Some(debug) = args.debug {
        if debug {
            pretty_print(&format!("Grep params: {:#?}", &params), Status::Info);
        }
    }

    let paths = grep(params);

    for path in paths {
        println!("{}", &path);
    }
}

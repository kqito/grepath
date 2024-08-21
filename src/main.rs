mod args;
mod grep;
mod output;

use std::io::{self, Read};

use args::Args;
use grep::grep;
use grep::params::GrepParamsBuilder;
use output::{pretty_print, pretty_println, Status};

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

    let content = if pipeline_input == "" {
        None
    } else {
        Some(pipeline_input)
    };

    let mut params_builder = GrepParamsBuilder::new()
        .no_validate(args.no_validate)
        .unique(args.unique)
        .content(content);

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
            pretty_println(&e.to_string(), Status::Error);

            eprintln!("Usage:");
            eprintln!("  $grepath <file>");
            eprintln!("\nExample:");
            eprintln!("  $grepath sample.txt");
            eprintln!("  $cat sample.txt | grepath");
            eprintln!("\nFor more details, please run:");
            eprintln!("  $grepath --help");
            return;
        }
    };

    if let Some(debug) = args.debug {
        if debug {
            pretty_print(&format!("Grep params: {:#?}", &params), Status::Info);
        }
    }

    let items = grep(&params);

    for item in items {
        println!("{}", &item.path);
    }
}

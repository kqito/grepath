mod args;
mod grep;
mod output;

use std::io::{self, Read};

use args::Args;
use grep::grep;
use grep::params::GrepParamsBuilder;
use output::{pretty_print, pretty_println, print_help, Status};

fn main() {
    let args: Args = argh::from_env();

    let mut pipeline_input = String::new();
    let is_piped = atty::isnt(atty::Stream::Stdin);

    if is_piped {
        if io::stdin().read_to_string(&mut pipeline_input).is_err() {
            pretty_print("Failed to read from stdin", Status::Error);
            std::process::exit(1);
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
                std::process::exit(1);
            }
        }
    }

    let params = match params_builder.build() {
        Ok(params) => params,
        Err(e) => {
            // If the program is piped, we don't want to print the error message
            if is_piped {
                std::process::exit(0);
            }

            pretty_println(&e.to_string(), Status::Error);
            print_help();
            std::process::exit(1);
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

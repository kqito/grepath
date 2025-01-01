mod args;
mod grep;
mod output;

use args::{get_input, Args};
use grep::grep;
use grep::params::GrepParamsBuilder;
use output::{pretty_print, pretty_println, print_help, Status};

fn main() {
    let args: Args = argh::from_env();

    let input = match get_input() {
        Ok(input) => input,
        Err(e) => {
            pretty_println(&e.to_string(), Status::Error);
            std::process::exit(1);
        }
    };

    let mut params_builder = GrepParamsBuilder::new()
        .debug(args.debug)
        .unique(args.unique)
        .content(input)
        .current_dir(args.current_dir)
        .ignore(args.ignore);

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
            let is_piped = atty::isnt(atty::Stream::Stdin);
            if is_piped {
                std::process::exit(0);
            }

            pretty_println(&e.to_string(), Status::Error);
            print_help();
            std::process::exit(1);
        }
    };

    if params.debug {
        pretty_print(&format!("Grep params: {:#?}", &params), Status::Info);
    }

    let items = grep(&params);

    if params.debug {
        pretty_print(&format!("Grep items: {:#?}", &items), Status::Info);
    }

    for item in items {
        println!("{}", &item.path);
    }
}

#[allow(unused)]
pub enum Status {
    Error,
    Warning,
    Success,
    Info,
}

// signle print function that passable status 'error' or 'warning' or nothing
pub fn pretty_print(message: &str, status: Status) {
    match status {
        Status::Error => println!("\x1b[31m[ERROR]\x1b[0m {}\n", message),
        Status::Warning => println!("\x1b[33m[WARNING]\x1b[0m {}\n", message),
        Status::Success => println!("{}\n", message),
        Status::Info => println!("\x1b[34m[INFO]\x1b[0m {}\n", message),
    }
}

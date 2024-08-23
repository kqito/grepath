#[allow(unused)]
pub enum Status {
    Error,
    Warning,
    Success,
    Info,
}

pub fn pretty_print(message: &str, status: Status) {
    match status {
        Status::Error => print!("\x1b[31m[ERROR]\x1b[0m {}\n", message),
        Status::Warning => print!("\x1b[33m[WARNING]\x1b[0m {}\n", message),
        Status::Success => print!("{}\n", message),
        Status::Info => print!("\x1b[34m[INFO]\x1b[0m {}\n", message),
    }
}
pub fn pretty_println(message: &str, status: Status) {
    match status {
        Status::Error => println!("\x1b[31m[ERROR]\x1b[0m {}\n", message),
        Status::Warning => println!("\x1b[33m[WARNING]\x1b[0m {}\n", message),
        Status::Success => println!("{}\n", message),
        Status::Info => println!("\x1b[34m[INFO]\x1b[0m {}\n", message),
    }
}

pub fn print_help() {
    println!("Usage:");
    println!("  $grepath <file>");
    println!("\nExample:");
    println!("  $grepath sample.txt");
    println!("  $cat sample.txt | grepath");
    println!("\nFor more details, please run:");
    println!("  $grepath --help");
}

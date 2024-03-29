use std::{env, process};

fn main() {
    let config = grep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parsed arguments: {err}");
        process::exit(1);
    });
    println!("query: {}", config.query);
    println!("path: {}", config.file_path);

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

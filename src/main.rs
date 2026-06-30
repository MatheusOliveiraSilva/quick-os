use std::env;
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), io::Error> {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: quick-os <config-path>");
            std::process::exit(2);
        }
    };

    let contents = read_config(&path)?;
    print!("{contents}");
    Ok(())
}

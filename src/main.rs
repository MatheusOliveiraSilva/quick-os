use std::env;
use std::fs;
use std::io;

struct Config {
    path: String,
    contents: String,
}

fn read_config(path: String) -> Result<Config, io::Error> {
    let contents = fs::read_to_string(&path)?;
    Ok(Config { path, contents })
}

fn main() -> Result<(), io::Error> {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: quick-os <config-path>");
            std::process::exit(2);
        }
    };

    let config = read_config(path)?;
    print!("{}", config.contents);
    Ok(())
}

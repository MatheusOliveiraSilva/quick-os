use std::fs;
use std::io;

const CONFIG_PATH: &str = "config.txt";

fn read_config() -> Result<String, io::Error> {
    fs::read_to_string(CONFIG_PATH)
}

fn main() -> Result<(), io::Error> {
    let contents = read_config()?;
    print!("{contents}");
    Ok(())
}

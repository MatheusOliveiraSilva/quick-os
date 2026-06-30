use std::fs;

fn main() {
    const CONFIG_PATH: &str = "config.txt";

    match fs::read_to_string(CONFIG_PATH) {
        Ok(contents) => {
            print!("{contents}");
        }
        Err(error) => {
            eprintln!("quick-os: could not read {CONFIG_PATH}: {error}");
            std::process::exit(1);
        }
    }
}

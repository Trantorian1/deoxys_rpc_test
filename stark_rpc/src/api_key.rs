use serde::Deserialize;
use std::io::{self, Read};
use std::fs::File;

#[derive(Deserialize)]
struct Config {
    api_key: String
}

pub fn load_api_key(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let config: Config = serde_json::from_str(&content)?;
    Ok(config.api_key)
}
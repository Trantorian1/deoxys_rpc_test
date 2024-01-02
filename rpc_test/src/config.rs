use anyhow::Context;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Config {
    pub alchemy: String,
    pub deoxys: String,
}

impl Config {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Could not deserialize test at {path} into Config"))?;

        Ok(config)
    }
}
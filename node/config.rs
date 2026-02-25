use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub chain_name: String,
}

pub fn load() -> Config {
    let data = fs::read_to_string("config/genesis.json")
        .expect("Unable to read genesis file");

    serde_json::from_str(&data)
        .expect("Invalid genesis format")
}

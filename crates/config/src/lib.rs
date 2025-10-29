pub mod structs;

use std::{error::Error, fs};

use crate::structs::BackendKitConfig;

pub fn load_config(path: &str) -> Result<BackendKitConfig, Box<dyn Error>> {
    let str = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read config file"));
    let config: BackendKitConfig =
        toml::from_str(&str).unwrap_or_else(|_| panic!("Failed to parse config file"));

    Ok(config)
}
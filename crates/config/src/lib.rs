pub mod structs;

use std::{error::Error, fs};

use crate::structs::BackendKitConfig;

pub fn load_config(path: &str) -> Result<BackendKitConfig, Box<dyn Error>> {
    let str = fs::read_to_string(path)?;
    let config: BackendKitConfig = toml::from_str(&str)?;
    Ok(config)
}
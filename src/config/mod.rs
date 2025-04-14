mod consts;

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::config::consts::{CONFIG_FILE, CONFIG_PATH};

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub cycle_per_frame: u32,
    pub default_color: String
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            cycle_per_frame: 10,
            default_color: String::from("#0D822C")
        }
    }

    fn get_file_path() -> PathBuf {
        dirs::home_dir().unwrap()
            .join(CONFIG_PATH)
            .join(CONFIG_FILE)
    }

    pub fn read() -> Result<Configuration, String> {
        let file_path = Self::get_file_path();
        let file_contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Error reading config file: {}", e));
            }
        };

        toml::from_str(&file_contents).unwrap_or_else(|e| Err(format!("Error parsing config file: {}", e)))
    }

    pub fn update(&self) {
        let config_path = Self::get_file_path();
        let config_to_str = toml::to_string(self).unwrap();

        fs::write(config_path, config_to_str).unwrap();
    }
}

mod consts;
mod errors;

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::consts::{CONFIG_FILE, CONFIG_PATH};
use crate::config::errors::ConfigError;

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub vm: VmOptions,
    pub debug: DebugOptions,
}

#[derive(Deserialize, Serialize)]
pub struct VmOptions {
    pub cycles_per_frame: u32,
}

#[derive(Deserialize, Serialize)]
pub struct DebugOptions {
    pub enable_debug_menu: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            vm: VmOptions {
                cycles_per_frame: 10,
            },
            debug: DebugOptions {
                enable_debug_menu: false,
            }
        }
    }
}

impl Configuration {
    fn get_file_path() -> PathBuf {
        dirs::home_dir().unwrap()
            .join(CONFIG_PATH)
            .join(CONFIG_FILE)
    }

    pub fn load() -> Result<Self, ConfigError> {
        let file_path = Self::get_file_path();

        if !file_path.exists() {
            let config = Configuration::default();
            config.update()?;

            return Ok(config);
        }

        Ok(Self::read_from_file(&file_path)?)
    }

    fn read_from_file(path: &PathBuf) -> Result<Configuration, ConfigError> {
        let file_contents = fs::read_to_string(path)
            .map_err(|_| ConfigError::NotReadable)?;

        match toml::from_str(&file_contents) {
            Ok(config) => Ok(config),
            Err(_) => Err(ConfigError::InvalidFileFormat),
        }
    }

    pub fn update(&self) -> Result<(), ConfigError> {
        let config_path = Self::get_file_path();

        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())
                .map_err(|_| ConfigError::CannotCreateDirectory)?;
        }

        let config_to_str = toml::to_string(self).unwrap();
        fs::write(config_path, config_to_str)
            .map_err(|_| ConfigError::NotWritable)?;

        Ok(())
    }
}

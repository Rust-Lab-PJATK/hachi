use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConfigError {
    InvalidFileFormat,
    NotReadable,
    NotWritable,
    CannotCreateDirectory
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ConfigError {}
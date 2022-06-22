use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::path::Path;

enum ConfigError {
    /// The config file was not found.
    NotFound,
    /// The config file was found, but could not be parsed.
    ParseError,
    /// The config file was found, but could not be loaded.
    LoadError,
}

// Window Configurations
struct Window {
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    maximized: bool,
}

// Database Configurations
struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

#[derive(Deserialize)]
pub struct Config {
    window: Option<Window>,
    database: Option<Database>
}


// TODO: Finish Config Setup
impl Config {
    pub fn new() -> Self {
        Config{}
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        if path.is_empty() {
            NotFound
        }

        let file = File::open(path).map_err(|_| NotFound)?;
        let reader = BufReader::new(file);

        let config: Config = serde_json::from_reader(reader).map_err(|_| ParseError)?;

        Ok(config)
    }
}

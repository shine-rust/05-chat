use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("app.yaml"),
            File::open("/etc/config/app.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(f), _, _) => serde_yaml::from_reader(f),
            (_, Ok(f), _) => serde_yaml::from_reader(f),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("No configuration file found"),
        };
        Ok(ret?)
    }
}

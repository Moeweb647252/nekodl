use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub password: String,
    pub username: String,
}

impl Config {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let content = read_to_string(path)?;
        let res = serde_json::from_str(&content)?;
        Ok(res)
    }

    pub fn update_password(self, new_pw: String) -> Self {
        Self {
            password: new_pw,
            ..self
        }
    }

    pub fn update_bind_addr(self, new_ba: String) -> Self {
        Self {
            bind_address: new_ba,
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "[::]:8001".to_owned(),
            username: "admin".to_owned(),
            password: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub token: Option<String>,
}

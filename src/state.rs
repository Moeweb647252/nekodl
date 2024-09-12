use anyhow::{Context, Ok, Result};
use salvo::Depot;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::rss::Rss;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub password: String,
    pub username: String,
    pub token: Option<String>,
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

    pub fn borrow_from(depot: &Depot) -> anyhow::Result<&Arc<RwLock<Self>>> {
        Ok(depot.obtain::<Arc<RwLock<Self>>>().ok().context(format!(
            "Internal Error: file: {},lLine: {}",
            file!(),
            line!()
        ))?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "[::]:8001".to_owned(),
            username: "admin".to_owned(),
            password: "".to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub token: Option<String>,
}

impl State {
    pub fn borrow_from(depot: &Depot) -> anyhow::Result<&Arc<RwLock<Self>>> {
        Ok(depot.obtain::<Arc<RwLock<Self>>>().ok().context(format!(
            "Internal Error: file: {},lLine: {}",
            file!(),
            line!()
        ))?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataBase {
    pub rss: Vec<Rss>,
}

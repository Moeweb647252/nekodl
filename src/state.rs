use anyhow::{Context, Ok, Result};
use salvo::Depot;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf, sync::Arc, time::Duration};
use tokio::{fs::write, sync::RwLock, time::sleep};

use crate::rss::Rss;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub password: String,
    pub username: String,
    pub token: Option<String>,
    pub db_path: String,
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
            token: None,
            db_path: "aria2.db".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DataBase {
    pub rss_list: Vec<Rss>,
}

impl DataBase {
    pub async fn save(&self, path: &str) -> Result<()> {
        let data = bincode::serialize(self)?;
        let path = PathBuf::from(path);
        tokio::fs::write(path, data).await?;
        Ok(())
    }

    pub fn rss_list_mut(&mut self) -> &mut Vec<Rss> {
        &mut self.rss_list
    }
}

pub async fn data_save_task(
    db: Arc<RwLock<DataBase>>,
    config: Arc<RwLock<Config>>,
    config_path: String,
) {
    loop {
        sleep(Duration::from_secs(60)).await;
        let config_dup = { config.read().await.clone() };
        write(
            &config_path,
            serde_json::to_string_pretty(&config_dup)
                .unwrap()
                .as_bytes(),
        )
        .await
        .unwrap();
        db.write().await.save(&config_dup.db_path).await.unwrap();
    }
}

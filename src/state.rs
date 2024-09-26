use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf, sync::Arc, time::Duration};
use tokio::{fs::write, sync::RwLock, time::sleep};
use tracing::info;

use crate::{download::DownloadTask, rss::Rss};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub password: String,
    pub username: String,
    pub token: Option<String>,
    pub db_path: String,
    pub session_path: String,
    pub torrent_options: TorrentOptions,
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
            db_path: "./db.bin".to_owned(),
            session_path: "./session".to_owned(),
            torrent_options: TorrentOptions {
                trackers: Vec::new(),
            },
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub token: Option<String>,
    pub rqbit_session: Option<Arc<librqbit::Session>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentOptions {
    pub trackers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DataBase {
    pub rss_list: Vec<Rss>,
    pub rss_id_index: usize,
    pub download_task_list: Vec<DownloadTask>,
}

impl DataBase {
    pub async fn save(&self, path: &str) -> Result<()> {
        info!("save db to {}", path);
        let data = bincode::serialize(self)?;
        let path = PathBuf::from(path);
        tokio::fs::write(path, data).await?;
        Ok(())
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

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Weak;
use std::{fs::read_to_string, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};
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
    pub output_path: String,
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
            output_path: "./downloads".to_owned(),
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

#[derive(Debug, Clone)]
pub struct SerdeLockLayer<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> SerdeLockLayer<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub async fn replace(&self, new: T) {
        *self.inner.write().await = new;
    }

    pub fn weak(&self) -> Weak<RwLock<T>> {
        Arc::downgrade(&self.inner)
    }

    pub async fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write().await
    }
}

pub trait CloneInner<T>
where
    T: Clone,
{
    async fn clone_inner(&self) -> T;
}

impl<T> CloneInner<T> for SerdeLockLayer<T>
where
    T: Clone,
{
    async fn clone_inner(&self) -> T {
        self.inner.read().await.clone()
    }
}

impl<T> CloneInner<Vec<T>> for Vec<&SerdeLockLayer<T>>
where
    T: Clone,
{
    async fn clone_inner(&self) -> Vec<T> {
        let mut res = Vec::new();
        for layer in self {
            res.push(layer.clone_inner().await);
        }
        res
    }
}

impl<T> From<T> for SerdeLockLayer<T> {
    fn from(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}

impl<T> Serialize for SerdeLockLayer<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        loop {
            match self.inner.try_write() {
                Ok(guard) => {
                    return serializer.serialize_some(&(*guard));
                }
                Err(_) => {}
            }
        }
    }
}

impl<'de, T> Deserialize<'de> for SerdeLockLayer<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match T::deserialize(deserializer) {
            Ok(t) => Ok(SerdeLockLayer::new(t)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataBase {
    pub rss_list: HashMap<usize, SerdeLockLayer<Rss>>,
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

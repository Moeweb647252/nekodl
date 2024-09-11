use std::path::PathBuf;

use anyhow::{Ok, Result};
use aria2_ws::{Client, Map, TaskOptions};
use tokio::fs::read;

#[derive(Clone)]
pub struct Arai2 {
    client: Client,
    options: Option<TaskOptions>,
}

impl Arai2 {
    pub async fn new(url: &str, token: Option<&str>, options: Option<TaskOptions>) -> Result<Self> {
        let client = Client::connect(url, token).await?;
        Ok(Self { client, options })
    }

    pub async fn download_torrent(&self, path: PathBuf, save_path: String) -> Result<()> {
        let content = read(path).await?;
        let options = if let Some(mut p_options) = self.options.clone() {
            p_options.out = Some(save_path);
            p_options
        } else {
            TaskOptions {
                out: Some(save_path),
                ..Default::default()
            }
        };
        self.client
            .add_torrent(&content, None, Some(options), None, None)
            .await?;
        Ok(())
    }

    pub async fn download_process(&self) {}
}

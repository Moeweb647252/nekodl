use std::sync::Arc;

use librqbit::{self, AddTorrent, AddTorrentOptions};
use salvo::hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, RwLock};

use crate::state::{Config, DataBase};

pub enum Command {
    AddTorrentFile(Vec<u8>, oneshot::Sender<usize>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {}

pub async fn download_command_task(
    mut receiver: mpsc::Receiver<Command>,
    db: Arc<RwLock<DataBase>>,
    config: Arc<RwLock<Config>>,
) -> anyhow::Result<()> {
    let mut session_path = dirs::template_dir().unwrap_or("/tmp".into());
    session_path.push("aria2rss");
    let session = librqbit::Session::new(session_path).await?;
    while let Some(command) = receiver.recv().await {
        match command {
            Command::AddTorrentFile(torrent_file, tx) => {
                let resp = session
                    .add_torrent(
                        AddTorrent::TorrentFileBytes(Bytes::from(torrent_file)),
                        Some(AddTorrentOptions {
                            trackers: Some(config.read().await.torrent_options.trackers.clone()),
                            ..Default::default()
                        }),
                    )
                    .await?;
                let handle = resp.into_handle().unwrap();
                tx.send(handle.id()).ok();
            }
        }
    }
    Ok(())
}

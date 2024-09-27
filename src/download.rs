use std::sync::Arc;

use librqbit::{self, AddTorrent, AddTorrentOptions};
use salvo::hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, RwLock};

use crate::state::{Config, DataBase, State};

pub enum Command {
    AddTorrentFile(Vec<u8>, oneshot::Sender<usize>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {}

pub async fn download_command_task(
    mut receiver: mpsc::Receiver<Command>,
    state: Arc<RwLock<State>>,
    config: Arc<RwLock<Config>>,
) -> anyhow::Result<()> {
    let session = librqbit::Session::new(config.read().await.session_path.as_str().into()).await?;
    state.write().await.rqbit_session = Some(session.clone());
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

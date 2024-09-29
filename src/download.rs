use std::{path::PathBuf, rc::Weak, sync::Arc, time::Duration};

use anyhow::{Context, Ok};
use librqbit::{self, AddTorrent, AddTorrentOptions, Session};
use salvo::hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot, RwLock},
    time::sleep,
};

use crate::{
    rss::{RssItem, RssItemStatus},
    state::{Config, State},
};

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

pub async fn item_downaload_task(
    session: Arc<Session>,
    item: Weak<RwLock<RssItem>>,
    title: String,
    config: Arc<RwLock<Config>>,
) -> anyhow::Result<()> {
    let (link, output_path) = {
        let item = item.upgrade().context("Can't upgrade item")?;
        let item = item.read().await;
        let mut output_path = PathBuf::from(&config.read().await.output_path);
        output_path.push(title);
        (item.link.clone(), output_path.to_string_lossy().into())
    };
    let resp = session
        .add_torrent(
            AddTorrent::Url(link.into()),
            Some(AddTorrentOptions {
                trackers: Some(config.read().await.torrent_options.trackers.clone()),
                output_folder: Some(output_path),
                ..Default::default()
            }),
        )
        .await?;
    let handle = resp.into_handle().unwrap();
    let weak = item.clone();
    tokio::select! {
        _ = handle.wait_until_completed() => {
            let lock = item.upgrade().context("Can't upgrade item")?;
            let mut guard = lock.write().await;
            guard.status = RssItemStatus::Downloaded;
        }
        _ = async move {
            loop {
                if weak.upgrade().is_none() {
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        } => {
            session.delete(handle.shared.info_hash.into(), false).await?;
        }
    };
    Ok(())
}

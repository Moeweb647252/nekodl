use std::{
    path::PathBuf,
    sync::{Arc, Weak},
    time::Duration,
};

use anyhow::{anyhow, Context, Ok};
use librqbit::{self, AddTorrent, AddTorrentOptions, ManagedTorrent, Session};
use salvo::{async_trait, hyper::body::Bytes};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot, RwLock},
    time::sleep,
};

use crate::{
    rss::{RssItem, RssItemStatus},
    state::{Config, State},
};

use super::{DownloadOptions, Downloader, Source};

pub enum Command {
    AddTorrentFile(Vec<u8>, oneshot::Sender<Arc<ManagedTorrent>>),
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
                tx.send(handle).ok();
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

pub struct Rqbit {
    session: Arc<Session>,
}

#[async_trait]
impl Downloader for Rqbit {
    async fn add_download_task(
        &self,
        source: super::Source,
        options: DownloadOptions,
    ) -> anyhow::Result<Arc<dyn super::DownloadHandle>> {
        use Source::*;
        match source {
            HttpUrl(url) => {
                return Err(anyhow!("Not implemented"));
            }
            MagnetLink(magnet_link) => {
                let add_torrent = AddTorrent::from_url(magnet_link);
                let resp = self.session.add_torrent(
                    add_torrent,
                    Some(AddTorrentOptions {
                        ..Default::default()
                    }),
                );

                todo!()
            }
            TorrentFile(bytes) => {
                todo!()
            }
        }
    }

    async fn cancel_download_task(
        &self,
        handle: Arc<dyn super::DownloadHandle>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_download_task_status(
        &self,
        handle: Arc<dyn super::DownloadHandle>,
    ) -> anyhow::Result<Box<dyn super::DownloadStatus>> {
        todo!()
    }

    async fn pause_download_task(
        &self,
        handle: Arc<dyn super::DownloadHandle>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn resume_download_task(
        &self,
        handle: Arc<dyn super::DownloadHandle>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

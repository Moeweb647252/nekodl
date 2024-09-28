use anyhow::{Context, Ok, Result};
use librqbit::{AddTorrent, AddTorrentOptions, Session};
use serde::Serialize;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;

use crate::rss::{ItemTorrent, RssItem, TorrentFileInfo};

#[derive(Serialize)]
pub struct TorrentInfo {
    files: Vec<FileInfo>,
}

#[derive(Serialize)]
pub struct FileInfo {
    filename: String,
    torrent_offset: u64,
    size: u64,
}

pub async fn fetch_torrent_info<'a>(
    add_torrent: AddTorrent<'a>,
    session: Arc<Session>,
    trackers: Vec<String>,
) -> Result<TorrentInfo> {
    let resp = session
        .add_torrent(
            add_torrent,
            Some(AddTorrentOptions {
                list_only: true,
                trackers: Some(trackers),
                ..Default::default()
            }),
        )
        .await?;
    let handle = resp
        .into_handle()
        .context("AddTorrentResponse.into_handle() failed")?;
    handle.wait_until_initialized().await?;
    Ok(TorrentInfo {
        files: handle
            .shared
            .file_infos
            .iter()
            .map(|f| FileInfo {
                filename: f.relative_filename.to_string_lossy().to_string(),
                torrent_offset: f.offset_in_torrent,
                size: f.len,
            })
            .collect(),
    })
}

pub async fn fetch_torrent_for_item<'a>(
    add_torrent: AddTorrent<'a>,
    session: Arc<Session>,
    trackers: Vec<String>,
    lock: Weak<RwLock<RssItem>>,
) -> Result<ItemTorrent> {
    let info = fetch_torrent_info(add_torrent, session, trackers).await?;
    let res = ItemTorrent {
        files: info
            .files
            .into_iter()
            .map(|f| TorrentFileInfo {
                filename: f.filename,
                offset: f.torrent_offset,
                length: f.size,
            })
            .collect(),
        update_time: std::time::SystemTime::now(),
    };
    lock.upgrade()
        .context("Can not upgread Weak")?
        .write()
        .await
        .torrent = Some(res.clone());
    Ok(res)
}

use anyhow::Result;
use std::sync::Arc;

mod rqbit;

pub enum Source {
    HttpUrl(String),
    MagnetLink(String),
    TorrentFile(Vec<u8>),
}

pub trait DownloadHandle {
    fn id(&self) -> usize;
}

pub trait DownloadStatus {}

pub enum DownloadOptions {
    Http { output_path: Option<String> },
    Torrent { trackers: Vec<String> },
}

pub trait Downloader {
    async fn add_download_task(
        &self,
        source: Source,
        options: DownloadOptions,
    ) -> Result<Arc<dyn DownloadHandle>>;

    async fn cancel_download_task(&self, handle: Arc<dyn DownloadHandle>) -> Result<()>;

    async fn pause_download_task(&self, handle: Arc<dyn DownloadHandle>) -> Result<()>;

    async fn resume_download_task(&self, handle: Arc<dyn DownloadHandle>) -> Result<()>;

    async fn get_download_task_status(
        &self,
        handle: Arc<dyn DownloadHandle>,
    ) -> Result<Box<dyn DownloadStatus>>;
}

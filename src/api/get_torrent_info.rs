use super::*;
use base64::Engine;
use librqbit::{AddTorrent, AddTorrentOptions};
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqData {
    url: Option<String>,
    bytes: Option<String>,
}

#[derive(Serialize)]
struct FileInfo {
    filename: String,
    torrent_offset: u64,
    size: u64,
}

#[derive(Serialize)]
struct RespData {
    files: Vec<FileInfo>,
}

#[handler]
pub async fn get_torrent_info(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<ApiResponse<RespData>, Error> {
    let reqdata: ReqData = req.parse_json().await?;
    let add_torrent = if let Some(url) = reqdata.url {
        AddTorrent::from_url(url)
    } else if let Some(bytes) = reqdata.bytes {
        AddTorrent::from_bytes(base64::prelude::BASE64_STANDARD.decode(bytes)?)
    } else {
        return Err(anyhow!("url or torrent file is required").into());
    };
    let session = StateLock::from_depot(&depot)?
        .read()
        .await
        .rqbit_session
        .clone()
        .context("Session not initialized")?;
    let resp = session
        .add_torrent(
            add_torrent,
            Some(AddTorrentOptions {
                list_only: true,
                ..Default::default()
            }),
        )
        .await?;
    Ok(ApiResponse::ok(RespData {
        files: resp
            .into_handle()
            .context("Failed to add torrent")?
            .shared
            .file_infos
            .iter()
            .map(|f| FileInfo {
                filename: f.relative_filename.to_string_lossy().to_string(),
                torrent_offset: f.offset_in_torrent,
                size: f.len,
            })
            .collect(),
    }))
}

use crate::{
    api::*,
    torrent::{fetch_torrent_info, TorrentInfo},
};
use base64::Engine;
use librqbit::AddTorrent;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqData {
    url: Option<String>,
    bytes: Option<String>,
}

#[handler]
pub async fn get_torrent_info(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<ApiResponse<TorrentInfo>, Error> {
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
    Ok(ApiResponse::ok(
        fetch_torrent_info(add_torrent, session, Vec::new()).await?,
    ))
}

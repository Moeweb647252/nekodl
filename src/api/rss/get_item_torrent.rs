use crate::{api::*, rss::ItemTorrent, torrent::fetch_torrent_for_item};
use librqbit::AddTorrent;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqData {
    rss_id: usize,
    item_id: usize,
}

#[handler]
pub async fn get_item_torrent(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<ApiResponse<ItemTorrent>, Error> {
    let reqdata: ReqData = req.parse_json().await?;
    let db = DataBaseLock::from_depot(&depot)?.read().await;
    let rss = db
        .rss_list
        .get(&reqdata.rss_id)
        .context("Rss not found")?
        .read()
        .await;
    let item = rss.items.get(reqdata.item_id).context("Item not found")?;
    let add_torrent = AddTorrent::Url(item.read().await.link.clone().into());
    let session = StateLock::from_depot(&depot)?
        .read()
        .await
        .rqbit_session
        .clone()
        .context("Session not initialized")?;
    Ok(ApiResponse::ok(
        fetch_torrent_for_item(add_torrent, session, Vec::new(), item.weak()).await?,
    ))
}

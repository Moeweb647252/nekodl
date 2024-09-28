use crate::download::Command;

use crate::api::*;
use crate::utils::FromDepot;
use base64::prelude::*;
use salvo::prelude::*;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

#[derive(Deserialize)]
struct ReqData {
    bt_data: String,
}

#[handler]
pub async fn add_torrent_task(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<()>, Error> {
    let data: ReqData = req.parse_json().await?;
    let bt_data = BASE64_STANDARD.decode(data.bt_data)?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    Sender::from_depot(&depot)?.send(Command::AddTorrentFile(bt_data, tx));
    todo!()
}

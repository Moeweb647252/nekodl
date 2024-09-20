use super::*;
use base64::prelude::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

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
    todo!()
}

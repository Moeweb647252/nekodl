use super::*;
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
) -> Result<ApiResponse<Option<()>>, Error> {
    todo!()
}

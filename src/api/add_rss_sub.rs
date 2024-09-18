use super::*;
use base64::prelude::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ReqData {
    name: String,
    url: String,
}

#[handler]
pub async fn add_rss_sub(depot: &mut Depot, req: &mut Request) -> Result<ApiResponse<()>, Error> {
    let data: ReqData = req.parse_json().await?;
    let rss_url = data.url;
    {
        let mut config = Config::borrow_from(depot)?.write().await;
    }
    Ok(ApiResponse::ok(()))
}

use crate::api::*;
use salvo::prelude::*;

#[handler]
pub async fn get_config(req: &mut Request, depot: &mut Depot) -> Result<ApiResponse<()>, Error> {
    let config = req.parse_json().await?;
    *ConfigLock::from_depot(&depot)?.write().await = config;
    Ok(ApiResponse::ok(()))
}

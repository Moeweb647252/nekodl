use crate::api::*;
use salvo::prelude::*;

#[handler]
pub async fn get_config(depot: &mut Depot) -> Result<ApiResponse<Config>, Error> {
    Ok(ApiResponse::ok(
        ConfigLock::from_depot(&depot)?.read().await.clone(),
    ))
}

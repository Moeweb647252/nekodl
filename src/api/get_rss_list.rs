use super::*;
use crate::{rss::Rss, state::DataBase};
use salvo::prelude::*;

#[derive(Serialize)]
struct Resp {
    rsses: Vec<Rss>,
}

#[handler]
pub async fn get_rss_list(depot: &mut Depot) -> Result<ApiResponse<Resp>, Error> {
    let db = DataBase::from_depot(depot)?;
    Ok(ApiResponse::new(
        Code::Success,
        Resp {
            rsses: db.read().await.rss.clone(),
        },
        "",
    ))
}

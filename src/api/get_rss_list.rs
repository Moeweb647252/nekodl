use super::*;
use crate::{rss::Rss, state::DataBase};
use salvo::prelude::*;
use time::OffsetDateTime;

#[derive(Serialize)]
struct RssInfo {
    title: String,
    description: String,
    update: OffsetDateTime,
    update_interval: usize,
    item_num: usize,
}

#[derive(Serialize)]
struct Resp {
    rss_list: Vec<RssInfo>,
}

#[handler]
pub async fn get_rss_list(depot: &mut Depot) -> Result<ApiResponse<Resp>, Error> {
    Ok(ApiResponse::new(
        Code::Success,
        Resp {
            rss_list: DataBase::from_depot(depot)?
                .read()
                .await
                .rss_list
                .iter()
                .map(|v| RssInfo {
                    title: v.title.clone(),
                    description: v.description.clone(),
                    update: OffsetDateTime::from(v.update_time),
                    item_num: v.items.len(),
                    update_interval: v.update_interval.as_secs() as usize,
                })
                .collect(),
        },
        "",
    ))
}

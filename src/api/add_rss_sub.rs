use std::time::{Duration, SystemTime};

use crate::{
    rss::{fetch_channel, fetch_rss, Rss, RssStatus},
    state::DataBase,
};

use super::*;
use bincode::config;
use rss::Channel;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqData {
    url: String,
}

#[handler]
pub async fn add_rss_sub(depot: &mut Depot, req: &mut Request) -> Result<ApiResponse<()>, Error> {
    let data: ReqData = req.parse_json().await?;
    let Channel {
        title, description, ..
    } = fetch_channel(&data.url).await?;
    let id = {
        let mut db = DataBase::from_depot(depot)?.write().await;
        db.rss_id_index += 1;
        db.rss_id_index
    };
    let rss = Rss {
        id: id,
        title: title,
        url: data.url,
        items: Vec::new(),
        description: description,
        update_time: SystemTime::now(),
        update_interval: Duration::from_secs(3600),
        status: RssStatus::Created,
    };
    {
        DataBase::from_depot(depot)?
            .write()
            .await
            .rss_list_mut()
            .push(rss);
    }
    Ok(ApiResponse::ok(()))
}

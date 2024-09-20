use std::time::{Duration, SystemTime};

use crate::{
    event::Event,
    rss::{fetch_channel, Rss, RssStatus},
    state::DataBase,
};

use super::*;
use rss::Channel;
use salvo::prelude::*;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

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
    depot
        .obtain::<Sender<Event>>()
        .ok()
        .context("No sender found")?
        .send(Event::AddRss(rss))
        .await?;
    Ok(ApiResponse::ok(()))
}

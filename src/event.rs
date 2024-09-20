use std::sync::Arc;

use tokio::sync::{mpsc::Receiver, RwLock};
use tracing::error;

use crate::{
    rss::{fetch_rss, Rss},
    state::{Config, DataBase},
};

pub enum Event {
    AddRss(Rss),
    SaveDatabase,
}
pub async fn event_handle_task(
    config: Arc<RwLock<Config>>,
    db: Arc<RwLock<DataBase>>,
    mut receiver: Receiver<Event>,
) {
    use Event::*;
    while let Some(event) = receiver.recv().await {
        match event {
            AddRss(rss) => {
                db.write().await.rss_list.push(rss);
            }
            SaveDatabase => {
                db.read()
                    .await
                    .save(config.read().await.db_path.as_str())
                    .await
                    .inspect_err(|e| error!("save database error: {}", e))
                    .ok();
            }
        }
    }
}

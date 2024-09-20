use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{mpsc::Receiver, RwLock},
    task::JoinHandle,
};
use tracing::error;

use crate::{
    rss::Rss,
    state::{Config, DataBase},
};

pub enum Event {
    AddRss(Rss),
    SaveDatabase,
    UpdateRss(Rss),
}

pub async fn event_handle_task(
    config: Arc<RwLock<Config>>,
    db: Arc<RwLock<DataBase>>,
    mut receiver: Receiver<Event>,
) {
    use Event::*;
    let task_pool: HashMap<usize, JoinHandle<()>> = HashMap::new();
    while let Some(event) = receiver.recv().await {
        match event {
            AddRss(rss) => {
                todo!()
            }
            SaveDatabase => {
                db.read()
                    .await
                    .save(config.read().await.db_path.as_str())
                    .await
                    .inspect_err(|e| error!("save database error: {}", e))
                    .ok();
            }
            UpdateRss(rss) => {
                if let Some(item) = db
                    .write()
                    .await
                    .rss_list
                    .iter_mut()
                    .find(|item| item.id == rss.id)
                {
                    *item = rss;
                }
            }
        }
    }
}

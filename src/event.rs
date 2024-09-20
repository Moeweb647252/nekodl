use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};
use tracing::{debug, error, info};

use crate::{
    rss::{rss_task, Rss},
    state::{Config, DataBase},
};

#[derive(Debug, Clone)]
pub enum Event {
    AddRss(Rss),
    SaveDatabase,
    UpdateRss(Rss),
}

pub async fn event_handle_task(
    config: Arc<RwLock<Config>>,
    db: Arc<RwLock<DataBase>>,
    sender: Sender<Event>,
    mut receiver: Receiver<Event>,
) {
    use Event::*;
    let mut task_pool: HashMap<usize, JoinHandle<()>> = HashMap::new();
    for rss in db.write().await.rss_list.iter() {
        let handle = tokio::spawn(rss_task(sender.clone(), rss.clone()));
        task_pool.insert(rss.id, handle);
    }
    while let Some(event) = receiver.recv().await {
        match event {
            AddRss(rss) => {
                info!("Adding rss: {}", rss.url);
                let handle = tokio::spawn(rss_task(sender.clone(), rss.clone()));
                task_pool.insert(rss.id, handle);
                db.write().await.rss_list.push(rss);
                sender.send(SaveDatabase).await.unwrap();
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

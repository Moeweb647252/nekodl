use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};
use tracing::{error, info};

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

/// 处理事件的异步任务函数。
/// 该函数负责监听事件，根据事件类型执行相应的操作，
///
/// # 参数
/// * `config` - 配置信息的读写锁引用。
/// * `db` - 数据库的读写锁引用。
/// * `sender` - 用于发送事件的通道发送端。
/// * `receiver` - 用于接收事件的通道接收端。
pub async fn event_handle_task(
    config: Arc<RwLock<Config>>,
    db: Arc<RwLock<DataBase>>,
    sender: Sender<Event>,
    mut receiver: Receiver<Event>,
) {
    use Event::*;
    // 创建一个任务池，用于存储和管理异步任务。
    let mut task_pool: HashMap<usize, JoinHandle<()>> = HashMap::new();
    // 遍历数据库中的RSS列表，并为每个RSS源创建一个异步任务。
    for rss in db.write().await.rss_list.iter() {
        let handle = tokio::spawn(rss_task(sender.clone(), rss.clone()));
        task_pool.insert(rss.read().await.id, handle);
    }
    // 循环接收事件，并根据事件类型执行相应的操作。
    while let Some(event) = receiver.recv().await {
        match event {
            // 添加新的RSS源。
            AddRss(rss) => {
                info!("Adding rss: {}", rss.url);
                let id = rss.id;
                let rss = Arc::new(RwLock::new(rss));
                // 为新RSS源创建异步任务。
                let handle = tokio::spawn(rss_task(sender.clone(), rss.clone()));
                // 将新任务添加到任务池。
                task_pool.insert(id, handle);
                // 将新RSS源添加到数据库。
                db.write().await.rss_list.push(rss);
                // 发送保存数据库的事件。
                sender.send(SaveDatabase).await.unwrap();
            }
            // 保存数据库。
            SaveDatabase => {
                // 读取数据库并保存到配置文件指定的路径。
                db.read()
                    .await
                    .save(config.read().await.db_path.as_str())
                    .await
                    .inspect_err(|e| error!("save database error: {}", e))
                    .ok();
            }
            // 更新RSS源。
            UpdateRss(rss) => {
                // 在数据库中查找匹配的RSS源，并更新它。
                if let Some(item) = db
                    .write()
                    .await
                    .rss_list_mut().await.iter()
                    .find(|item| item.id == rss.id)
                {
                    **item = rss;
                }
            }
        }
    }
}

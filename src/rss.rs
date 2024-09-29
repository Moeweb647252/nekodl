use crate::state::{Config, SerdeLockLayer, State};
use crate::torrent::fetch_torrent_for_item;
use anyhow::{Context, Result};
use librqbit::AddTorrent;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub status: RssItemStatus,
    pub torrent: Option<ItemTorrent>,
    pub id: usize,
    #[serde(skip)]
    pub download_handle: Option<Arc<JoinHandle<()>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemTorrent {
    pub files: Vec<TorrentFileInfo>,
    pub update_time: std::time::SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TorrentFileInfo {
    pub filename: String,
    pub offset: u64,
    pub length: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RssItemStatus {
    Unread,
    Read,
    Downloading,
    Downloaded,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RssStatus {
    Read,
    Created,
    Updated,
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rss {
    pub id: usize,
    pub url: String,
    pub title: String,
    pub description: String,
    pub items: Vec<SerdeLockLayer<RssItem>>,
    pub update_time: std::time::SystemTime,
    pub update_interval: std::time::Duration,
    pub status: RssStatus,
    pub auto_download: bool,
}

impl Rss {
    pub fn info(&self) -> Self {
        Self {
            items: Vec::new(),
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            update_time: self.update_time,
            update_interval: self.update_interval,
            status: RssStatus::Created,
            auto_download: self.auto_download,
        }
    }
}

pub async fn fetch_channel(link: &str) -> Result<Channel> {
    let client = reqwest::Client::new();
    let content = client.get(link).send().await?.bytes().await?;
    Ok(Channel::read_from(&content[..])?)
}

// 定义一个异步函数rss_task，用于更新RSS源并发送更新事件
// sender: 用于发送事件的通道
// rss: 需要更新的RSS源
pub async fn rss_task(
    rss_lock: Weak<RwLock<Rss>>,
    state: Arc<RwLock<State>>,
    config: Arc<RwLock<Config>>,
) {
    // 无限循环，持续检查并更新RSS源
    loop {
        let rss = if let Some(lock) = rss_lock.upgrade() {
            lock.read().await.clone()
        } else {
            break;
        };
        // 如果距离上次更新时间小于更新间隔，并且RSS状态不是已创建，则等待剩余时间
        if rss.update_time.elapsed().unwrap() < rss.update_interval
            && rss.status != RssStatus::Created
        {
            sleep(rss.update_interval - rss.update_time.elapsed().unwrap()).await;
        }
        // 异步获取RSS源的频道信息
        let channel = fetch_channel(&rss.url).await.unwrap();
        // 初始化一个向量用于存储RSS项
        let mut items = Vec::new();
        // 遍历频道中的每一项
        for item in channel.items().iter().enumerate() {
            // 获取标题，如果没有标题则使用默认值"Default Title"
            let title = if let Some(title) = item.1.title() {
                title.to_string()
            } else {
                "Default Title".to_owned()
            };
            // 获取链接，如果没有链接则跳过该项
            let link = if let Some(link) = &item.1.enclosure {
                link.url.to_owned()
            } else {
                continue;
            };
            // 获取描述，如果没有描述则使用默认值"Default Description"
            let description = if let Some(description) = item.1.description() {
                description.to_string()
            } else {
                "Default Description".to_owned()
            };
            // 将获取到的RSS项信息添加到向量中
            items.push(RssItem {
                title,
                link,
                description,
                status: RssItemStatus::Unread,
                id: item.0,
                torrent: None,
                download_handle: None,
            });
        }
        for i in rss.items.iter() {
            let tmp = i.read().await;
            if let Some(pos) = items.iter().position(|x| tmp.comprare(x)) {
                items.remove(pos);
            }
        }
        let mut index = rss.items.len();
        items.iter_mut().for_each(|v| {
            v.id = index;
            index += 1;
        });

        if let Some(lock) = rss_lock.upgrade() {
            {
                let mut guard = lock.write().await;
                guard.update_time = std::time::SystemTime::now();
                for i in items {
                    guard.items.push(i.into());
                }
                guard.status = RssStatus::Updated;
            }
            let guard = lock.read().await;
            for i in guard.items.iter() {
                let item = i.read().await.clone();
                if let Some(torrent) = item.torrent {
                    if rss.auto_download {}
                } else {
                    let weak = i.weak();
                    let session = state
                        .read()
                        .await
                        .rqbit_session
                        .clone()
                        .context("librqbit session not found")
                        .unwrap();
                    let trackers = config.read().await.torrent_options.trackers.clone();
                    tokio::spawn(async move {
                        fetch_torrent_for_item(
                            AddTorrent::Url(item.link.into()),
                            session,
                            trackers,
                            weak,
                        )
                        .await
                    });
                }
            }
        } else {
            break;
        };
    }
}

impl RssItem {
    pub fn comprare(&self, item: &Self) -> bool {
        self.title == item.title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::main]
    async fn async_test_fetch_rss() {
        println!(
            "{:?}",
            fetch_channel("https://mikanani.me/RSS/Bangumi?bangumiId=3367&subgroupid=611")
                .await
                .unwrap()
                .items()[0]
        );
    }

    #[test]
    fn test_fetch_rss() {
        async_test_fetch_rss()
    }
}

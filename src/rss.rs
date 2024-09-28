use std::sync::Arc;
use anyhow::Result;
use rss::Channel;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::event::Event;

use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub status: RssItemStatus,
    pub torrent: Option<ItemTorrent>,
    pub id: usize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemTorrent {
    pub link: String,
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
    pub items: Vec<RssItem>,
    pub update_time: std::time::SystemTime,
    pub update_interval: std::time::Duration,
    pub status: RssStatus,
}

pub async fn fetch_channel(link: &str) -> Result<Channel> {
    let client = reqwest::Client::new();
    let content = client.get(link).send().await?.bytes().await?;
    Ok(Channel::read_from(&content[..])?)
}

// 定义一个异步函数rss_task，用于更新RSS源并发送更新事件
// sender: 用于发送事件的通道
// rss: 需要更新的RSS源
pub async fn rss_task(sender: Sender<Event>, mut rss_lock: Arc<RwLock<Rss>>) {
    // 无限循环，持续检查并更新RSS源
    loop {
        let mut rss = rss_lock.read().await.clone();
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
            let link = if let Some(link) = item.1.link() {
                link.to_string()
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
            });
        }
        // 更新RSS源的项
        rss.items = items;
        // 更新RSS源的最后更新时间
        rss.update_time = std::time::SystemTime::now();
        // 设置RSS源的状态为已更新
        rss.status = RssStatus::Updated;
        // 发送更新事件
        sender.send(Event::UpdateRss(rss.clone())).await.unwrap();
    }
}

impl RssItem {
    pub fn comprare_rss_crate(self, item: &rss::Item) -> bool {
        Some(self.title.as_str()) == item.title()
            && Some(self.link.as_str()) == item.link()
            && Some(self.description.as_str()) == item.description()
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
                .items()
        );
    }

    #[test]
    fn test_fetch_rss() {
        async_test_fetch_rss()
    }
}

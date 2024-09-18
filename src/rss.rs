use std::sync::Arc;

use anyhow::Result;
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::state::{Config, DataBase};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub status: RssItemStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RssItemStatus {
    Unread,
    Read,
    Downloading,
    Downloaded,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RssStatus {
    Created,
    Updated,
    Error(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rss {
    pub url: String,
    pub title: String,
    pub description: String,
    pub items: Vec<RssItem>,
    pub update_time: std::time::SystemTime,
    pub update_interval: std::time::Duration,
    pub status: RssStatus,
}

pub async fn fetch_rss(link: &str) -> Result<Channel> {
    let client = reqwest::Client::new();
    let content = client.get(link).send().await?.bytes().await?;
    Ok(Channel::read_from(&content[..])?)
}

pub async fn rss_task(db: Arc<RwLock<DataBase>>) {
    loop {
        let mut db = db.write().await; //锁一直锁
        for rss in db.rss_list.iter_mut() {
            if rss.update_time.elapsed().unwrap() > rss.update_interval {
                //耗时操作 fetch_rss
                let channel = match fetch_rss(&rss.url).await {
                    Ok(channel) => channel,
                    Err(e) => {
                        rss.status = RssStatus::Error(e.to_string());
                        continue;
                    }
                };
                rss.status = RssStatus::Updated;
                rss.update_time = std::time::SystemTime::now();
                for item in channel.items() {
                    let item = item.clone();
                    if let Some(link) = item.link {
                        if rss.items.iter().filter(|i| i.link == link).count() == 0 {
                            //此处是写操作
                            rss.items.push(RssItem {
                                title: item.title.clone().unwrap_or("default title".to_owned()),
                                link,
                                description: item
                                    .description
                                    .clone()
                                    .unwrap_or("default description".to_owned()),
                                status: RssItemStatus::Unread,
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::fetch_rss;

    #[tokio::main]
    async fn async_test_fetch_rss() {
        println!(
            "{:?}",
            fetch_rss("https://mikanani.me/RSS/Bangumi?bangumiId=3367&subgroupid=611")
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

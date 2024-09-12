use std::sync::Arc;

use anyhow::Result;
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::state::{Config, DataBase};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RssItem {
    title: String,
    link: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rss {
    url: String,
    title: String,
    description: String,
    items: Vec<RssItem>,
    update_time: std::time::SystemTime,
    update_interval: std::time::Duration,
}

pub async fn fetch_rss(link: &str) -> Result<Channel> {
    let client = reqwest::Client::new();
    let content = client.get(link).send().await?.bytes().await?;
    Ok(Channel::read_from(&content[..])?)
}

pub async fn rss_task(db: Arc<RwLock<DataBase>>) {
    loop {
        let rsses = { db.read().await.rss.clone() };
        for rss in rsses {
            if rss.update_time.elapsed().unwrap() > rss.update_interval {}
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

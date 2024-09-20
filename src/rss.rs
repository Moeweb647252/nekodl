use anyhow::Result;
use rss::Channel;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::event::Event;

use tokio::sync::mpsc::Sender;

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

pub async fn rss_task(sender: Sender<Event>, mut rss: Rss) {
    loop {
        if rss.update_time.elapsed().unwrap() < rss.update_interval
            && rss.status != RssStatus::Created
        {
            sleep(rss.update_interval - rss.update_time.elapsed().unwrap()).await
        }
        let channel = fetch_channel(&rss.url).await.unwrap();
        let mut items = Vec::new();
        for item in channel.items() {
            let title = if let Some(title) = item.title() {
                title.to_string()
            } else {
                "Default Title".to_owned()
            };
            let link = if let Some(link) = item.link() {
                link.to_string()
            } else {
                continue;
            };
            let description = if let Some(description) = item.description() {
                description.to_string()
            } else {
                "Default Description".to_owned()
            };
            items.push(RssItem {
                title,
                link,
                description,
                status: RssItemStatus::Unread,
            });
        }
        rss.items = items;
        rss.update_time = std::time::SystemTime::now();
        rss.status = RssStatus::Updated;
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

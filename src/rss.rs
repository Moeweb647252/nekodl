use anyhow::Result;
use rss::Channel;

pub async fn fetch_rss(link: &str) -> Result<Channel> {
    let client = reqwest::Client::new();
    let content = client.get(link).send().await?.bytes().await?;
    Ok(Channel::read_from(&content[..])?)
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

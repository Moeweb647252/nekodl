use std::time::{Duration, SystemTime};

use crate::{
    event::Event,
    rss::{fetch_channel, Rss, RssStatus},
};

use crate::api::*;
use ::rss::Channel;
use salvo::prelude::*;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

#[derive(Deserialize)]
struct ReqData {
    url: String,
    auto_download: bool,
}

#[handler]
/// 添加RSS订阅的处理函数
///
/// # 参数
/// * `depot` - 存储库引用，用于读写数据
/// * `req` - 请求引用，包含客户端请求的数据
///
/// # 返回值
/// * `Result<ApiResponse<()>, Error>` - 成功时返回空的API响应，失败时返回错误
pub async fn add_rss_sub(depot: &mut Depot, req: &mut Request) -> Result<ApiResponse<()>, Error> {
    // 解析请求中的JSON数据
    let data: ReqData = req.parse_json().await?;

    // 获取RSS源的信息
    let Channel {
        title, description, ..
    } = fetch_channel(&data.url).await?;

    // 生成新的RSS ID
    let id = {
        let mut db = DataBaseLock::from_depot(depot)?.write().await;
        db.rss_id_index += 1;
        db.rss_id_index
    };

    // 创建RSS对象
    let rss = Rss {
        id: id,
        title: title,
        url: data.url,
        items: Vec::new(),
        description: description,
        update_time: SystemTime::now(),
        update_interval: Duration::from_secs(3600),
        status: RssStatus::Created,
        auto_download: data.auto_download,
    };

    // 发送添加RSS的事件
    depot
        .obtain::<Sender<Event>>()
        .ok()
        .context("No sender found")?
        .send(Event::AddRss(rss))
        .await?;

    // 返回成功的API响应
    Ok(ApiResponse::ok(()))
}

use crate::api::*;
use salvo::prelude::*;
use time::OffsetDateTime;

#[derive(Serialize)]
struct RssInfo {
    title: String,
    description: String,
    update: OffsetDateTime,
    update_interval: usize,
    item_num: usize,
    id: usize,
}

#[derive(Serialize)]
struct Resp {
    rss_list: Vec<RssInfo>,
}

/// 获取RSS列表的处理函数。
/// 该函数从数据库中读取RSS列表，并将其转换为响应格式。
/// # Arguments
/// * `depot` - 一个可变的Depot引用，用于访问数据存储。
/// # Returns
/// * `Result<ApiResponse<Resp>, Error>` - 如果成功，则返回包含RSS列表的响应；如果失败，则返回错误。
#[handler]
pub async fn get_rss_list(depot: &mut Depot) -> Result<ApiResponse<Resp>, Error> {
    // 创建一个新的ApiResponse对象，状态码为成功
    Ok(ApiResponse::new(
        Code::Success,
        // 从Depot中读取数据库，并映射RSS列表项为RssInfo结构体
        Resp {
            rss_list: DataBaseLock::from_depot(depot)?
                .read()
                .await
                .rss_list().await
                .iter()
                .map(|v| RssInfo {
                    title: v.title.clone(),                                // 克隆标题
                    description: v.description.clone(),                    // 克隆描述
                    update: OffsetDateTime::from(v.update_time), // 从更新时间创建OffsetDateTime
                    item_num: v.items.len(),                     // 获取RSS项的数量
                    update_interval: v.update_interval.as_secs() as usize, // 将更新间隔转换为秒
                    id: v.id,
                })
                .collect(), // 收集映射后的RssInfo结构体为Vec
        },
        "", // 响应消息为空字符串
    ))
}

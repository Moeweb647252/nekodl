use crate::api::*;
use crate::rss::Rss;
use crate::state::CloneInner;
use salvo::prelude::*;
use time::OffsetDateTime;

#[derive(Serialize)]
struct Resp {
    rss_list: Vec<Rss>,
}

/// 获取RSS列表的处理函数。
/// 该函数从数据库中读取RSS列表，并将其转换为响应格式。
/// # Arguments
/// * `depot` - 一个可变的Depot引用，用于访问数据存储。
/// # Returns
/// * `Result<ApiResponse<Resp>, Error>` - 如果成功，则返回包含RSS列表的响应；如果失败，则返回错误。
#[handler]
pub async fn get_rss_list(depot: &mut Depot) -> Result<ApiResponse<Resp>, Error> {
    let mut res = Vec::new();
    // 从Depot中读取数据
    for i in DataBaseLock::from_depot(depot)?
        .read()
        .await
        .rss_list
        .values()
    {
        res.push(i.read().await.info());
    }
    // 创建一个新的ApiResponse对象，状态码为成功
    Ok(ApiResponse::new(
        Code::Success,
        Resp { rss_list: res },
        "", // 响应消息为空字符串
    ))
}

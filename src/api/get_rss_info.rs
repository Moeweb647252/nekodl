use crate::rss::Rss;

use super::*;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqData {
    id: usize,
}
//获取RSS信息
#[handler]
pub async fn get_rss_info(depot: &mut Depot, req: &mut Request) -> Result<ApiResponse<Rss>, Error> {
    // 解析请求中的JSON数据
    let data: ReqData = req.parse_json().await?;

    // 从数据库中读取RSS列表，并查找与请求数据ID匹配的RSS项
    // 如果找到，则返回该项的克隆；如果没有找到，则返回错误信息
    Ok(ApiResponse::ok(
        DataBaseLock::from_depot(depot)?
            .read()
            .await
            .rss_list
            .iter()
            .find(|v| v.id == data.id)
            .context("没有找到对应的RSS")?
            .clone(),
    ))
}

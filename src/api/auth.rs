use super::*;
use salvo::prelude::*;

#[handler]
pub async fn auth() -> Result<ApiResponse<()>, Error> {
    Ok(ApiResponse::new(Code::Success, (), ""))
}

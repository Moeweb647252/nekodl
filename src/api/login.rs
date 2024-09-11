use crate::state::Config;
use anyhow::{Context, Result};
use salvo::prelude::*;

use super::{ApiResponse, Code};

#[handler]
async fn login(depot: &mut Depot, req: &mut Request) -> Result<ApiResponse<Option<()>>> {
    let config = depot.obtain::<Config>().ok().context("")?;
    let username = req.form::<String>("username").await.context("Username")?;
    let password = req.form::<String>("password").await.context("Password")?;
    Ok(
        if config.password == password && config.username == username {
            ApiResponse::new(Code::Success, None, "")
        } else {
            ApiResponse::new(Code::AuthenticationError, None, "")
        },
    )
}

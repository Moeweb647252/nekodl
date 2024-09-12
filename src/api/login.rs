use crate::utils::rand_str;

use super::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Resp {
    pub token: String,
}

#[derive(Deserialize)]
struct ReqData {
    username: String,
    password: String,
}

#[handler]
pub async fn login(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<Option<Resp>>, Error> {
    let config = Config::borrow_from(depot)?.read().await;
    let json: ReqData = req.parse_json().await?;
    let token = rand_str(16);
    {
        let mut state = State::borrow_from(depot)?.write().await;
        state.token = Some(token.clone());
    }
    Ok(
        if config.password == json.password && config.username == json.username {
            ApiResponse::new(Code::Success, Some(Resp { token }), "")
        } else {
            ApiResponse::new(
                Code::AuthenticationError,
                None,
                "Username or password is incorrect",
            )
        },
    )
}

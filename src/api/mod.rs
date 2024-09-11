use anyhow::anyhow;
use salvo::{async_trait, Depot, Request, Response, Router, Writer};
use serde::Serialize;
use std::{fmt::Display, sync::Arc};
use tokio::sync::RwLock;
use tracing::error;

type Config = Arc<RwLock<crate::state::Config>>;
type State = Arc<RwLock<crate::state::State>>;

mod add_bt_task;
mod login;

pub fn routes() -> Vec<Router> {
    vec![Router::with_path("login").post(login::login)]
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    code: Code,
    data: T,
    msg: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: Code, data: T, msg: &str) -> Self {
        Self {
            code,
            data,
            msg: msg.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Code {
    Success,
    AuthenticationError,
    ServerError,
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(match self {
            Self::Success => 200,
            Self::AuthenticationError => 501,
            Self::ServerError => 502,
        })
    }
}

pub struct Error {
    pub inner: anyhow::Error,
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

pub trait Context<T> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;
}

impl<T> Context<T> for Option<T> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        if let Some(data) = self {
            Ok(data)
        } else {
            Err(anyhow!("Missing {}", context).into())
        }
    }
}

#[async_trait]
impl Writer for Error {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.body(
            match serde_json::to_string(&ApiResponse::new(
                Code::ServerError,
                Option::<()>::None,
                self.inner.to_string().as_str(),
            )) {
                Ok(data) => data,
                Err(err) => {
                    #[cfg(debug_assertions)]
                    error!("{}", err);
                    "Error".to_owned()
                }
            },
        );
    }
}

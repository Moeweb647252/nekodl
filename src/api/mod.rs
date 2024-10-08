use anyhow::anyhow;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response, Router, Writer};
use serde::Serialize;
use std::{fmt::Display, sync::Arc, vec};
use tokio::sync::RwLock;
use tracing::error;

use crate::{
    state::{Config, DataBase, State},
    utils::FromDepot,
};

mod auth;
mod config;
mod download;
mod login;
mod rss;

type DataBaseLock = Arc<RwLock<DataBase>>;
type StateLock = Arc<RwLock<State>>;
type ConfigLock = Arc<RwLock<Config>>;

pub fn routes() -> Vec<Router> {
    vec![
        Router::with_path("login").post(login::login),
        Router::new().hoop(ApiHandler).append(&mut vec![
            Router::with_path("add_torrent_task")
                .post(download::add_torrent_task::add_torrent_task),
            Router::with_path("auth").get(auth::auth),
            Router::with_path("get_rss_list").get(rss::get_rss_list::get_rss_list),
            Router::with_path("add_rss_sub").post(rss::add_rss_sub::add_rss_sub),
            Router::with_path("get_rss_info").post(rss::get_rss_info::get_rss_info),
        ]),
    ]
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

    pub fn ok(data: T) -> Self {
        Self::new(Code::Success, data, "Success")
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
            serde_json::to_string(&ApiResponse::new(
                Code::ServerError,
                Option::<()>::None,
                self.inner.to_string().as_str(),
            ))
            .unwrap_or_else(|err| {
                #[cfg(debug_assertions)]
                error!("{}", err);
                "Error".to_owned()
            }),
        );
    }
}

struct ApiHandler;

#[async_trait]
impl Handler for ApiHandler {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        #[cfg(debug_assertions)]
        {
            ctrl.call_next(req, depot, res).await;
            return;
        }
        #[cfg(not(debug_assertions))]
        {
            let state = match StateLock::from_depot(&depot) {
                Ok(state) => state.clone(),
                Err(err) => {
                    Error::from(err).write(req, depot, res).await;
                    ctrl.skip_rest();
                    return;
                }
            };

            let token = req
                .headers()
                .get("Token")
                .map(|v| v.to_str().unwrap_or(""))
                .unwrap_or("");
            if token
                != match &state.read().await.token {
                    Some(token) => token,
                    None => {
                        Error::from(anyhow!("Missing token"))
                            .write(req, depot, res)
                            .await;
                        ctrl.skip_rest();
                        return;
                    }
                }
            {
                Error::from(anyhow!("Invalid token"))
                    .write(req, depot, res)
                    .await;
                ctrl.skip_rest();
            } else {
                ctrl.call_next(req, depot, res).await;
            }
        }
    }
}

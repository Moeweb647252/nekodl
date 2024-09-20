use bincode::config;
use clap::Parser;
use event::event_handle_task;
use rss::rss_task;
use salvo::cors::{AllowCredentials, AllowHeaders, AllowMethods, Cors};
use salvo::prelude::*;
use state::{data_save_task, Config, DataBase, State};
use std::fs::{self, read};
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::{fs::write, sync::RwLock};
use tracing::info;
use tracing_subscriber::EnvFilter;
use utils::{rand_str, sha256};

mod api;
mod aria2;
mod event;
mod rss;
mod state;
mod static_serv;
mod utils;

#[derive(clap::Parser)]
struct App {
    #[arg(short = 'b')]
    bind: Option<String>,
    #[arg(short = 'c')]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = App::parse();

    let mut config = if let Some(conf_path) = &app.config {
        info!("Read config from {}", conf_path);
        Config::from_path(conf_path.into())?
    } else {
        info!("Creating default config");
        let rand_pw = rand_str(8);
        let sha_256_pw = sha256(&rand_pw);
        println!("Default account: username: admin, password: {}", rand_pw);
        let config = Config::default().update_password(sha_256_pw);
        write("./config.json", serde_json::to_string(&config)?).await?;
        config
    };

    if let Some(bind_addr) = &app.bind {
        config = config.update_bind_addr(bind_addr.clone())
    }

    let db_data = read(config.db_path.as_str());

    let db: DataBase = match db_data {
        Ok(data) => bincode::deserialize(&data)?,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                let db = DataBase {
                    rss_id_index: 0,
                    rss_list: Vec::new(),
                };
                let data = bincode::serialize(&db)?;
                write(config.db_path.as_str(), data).await?;
                db
            }
            _ => {
                panic!("Read database error: {}", e);
            }
        },
    };

    let state = Arc::new(RwLock::new(State { token: None }));
    let config = Arc::new(RwLock::new(config));
    let db = Arc::new(RwLock::new(db));
    tokio::spawn(data_save_task(
        db.clone(),
        config.clone(),
        app.config.unwrap_or("./config.json".to_owned()),
    ));
    let (tx, rx) = mpsc::channel(1000);
    tokio::spawn(event_handle_task(
        config.clone(),
        db.clone(),
        tx.clone(),
        rx,
    ));
    let sock = TcpListener::new("[::]:8001").bind().await;
    let router = Router::new()
        .hoop(affix_state::inject(tx.clone()))
        .hoop(affix_state::inject(config))
        .hoop(affix_state::inject(state))
        .hoop(affix_state::inject(db))
        .push(Router::with_path("/api").append(&mut api::routes()));
    let service = Service::new(router).hoop(
        Cors::new()
            .allow_origin("*")
            .allow_methods(AllowMethods::any())
            .allow_credentials(AllowCredentials::judge(|_, _, _| true))
            .allow_headers(AllowHeaders::any())
            .into_handler(),
    );

    Server::new(sock).serve(service).await;
    Ok(())
}

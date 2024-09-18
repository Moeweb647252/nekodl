use clap::Parser;
use salvo::cors::{AllowCredentials, AllowHeaders, AllowMethods, Cors};
use salvo::prelude::*;
use state::{Config, DataBase, State};
use std::fs::read;
use std::sync::Arc;
use tokio::{fs::write, sync::RwLock};
use utils::{rand_str, sha256};

mod api;
mod aria2;
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
    tracing_subscriber::fmt().init();

    let app = App::parse();

    let mut config = if let Some(conf_path) = app.config {
        Config::from_path(conf_path.into())?
    } else {
        let rand_pw = rand_str(8);
        let sha_256_pw = sha256(&rand_pw);
        println!("Default account: username: admin, password: {}", rand_pw);
        let config = Config::default().update_password(sha_256_pw);
        write("./config.json", serde_json::to_string(&config)?).await?;
        config
    };

    if let Some(bind_addr) = app.bind {
        config = config.update_bind_addr(bind_addr)
    }

    let db_data = read(config.db_path.as_str())?;
    let db: DataBase = bincode::deserialize(&db_data)?;

    let state = State { token: None };

    let sock = TcpListener::new("[::]:8001").bind().await;
    let router = Router::new()
        .hoop(affix_state::inject(Arc::new(RwLock::new(config))))
        .hoop(affix_state::inject(Arc::new(RwLock::new(state))))
        .hoop(affix_state::inject(Arc::new(RwLock::new(db))))
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

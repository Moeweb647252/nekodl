use clap::Parser;
use salvo::prelude::*;
use state::Config;
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
        Config::default().update_password(sha_256_pw)
    };
    if let Some(bind_addr) = app.bind {
        config = config.update_bind_addr(bind_addr)
    }
    let sock = TcpListener::new("[::]:8001").bind().await;
    Server::new(sock)
        .serve(
            Router::new()
                .hoop(affix_state::inject(config))
                .push(Router::with_path("/api").append(&mut api::routes())),
        )
        .await;
    Ok(())
}

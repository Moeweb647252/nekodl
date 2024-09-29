#![allow(dead_code)]
use clap::Parser;
use event::event_handle_task;
use salvo::cors::{AllowCredentials, AllowHeaders, AllowMethods, Cors};
use salvo::prelude::*;
use state::{data_save_task, Config, DataBase, State};
use std::collections::HashMap;
use std::fs::read;
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::{fs::write, sync::RwLock};
use tracing::info;
use tracing_subscriber::EnvFilter;
use utils::{rand_str, sha256};

mod api;
mod download;
mod event;
mod rss;
mod state;
mod static_serv;
mod task;
mod torrent;
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
    // 初始化日志记录器
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // 解析命令行参数
    let app = App::parse();

    // 加载或创建配置文件
    let mut config = if let Some(conf_path) = &app.config {
        info!("从 {} 读取配置", conf_path);
        Config::from_path(conf_path.into())?
    } else {
        info!("创建默认配置");
        let rand_pw = rand_str(8); // 生成随机密码
        let sha_256_pw = sha256(&rand_pw); // 对密码进行 SHA-256 哈希
        println!("默认账户: 用户名: admin, 密码: {}", rand_pw);
        let config = Config::default().update_password(sha_256_pw);
        write("./config.json", serde_json::to_string(&config)?).await?; // 将默认配置写入文件
        config
    };

    // 如果命令行参数中指定了绑定地址，则更新配置
    if let Some(bind_addr) = &app.bind {
        config = config.update_bind_addr(bind_addr.clone())
    }

    // 读取数据库数据
    let db_data = read(config.db_path.as_str());

    // 反序列化数据库数据或创建新的数据库实例
    let db: DataBase = match db_data {
        Ok(data) => bincode::deserialize(&data)?,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                let db = DataBase {
                    rss_id_index: 0,
                    rss_list: HashMap::new(),
                    download_task_list: Vec::new(),
                };
                let data = bincode::serialize(&db)?;
                write(config.db_path.as_str(), data).await?; // 将新的数据库实例写入文件
                db
            }
            _ => {
                panic!("读取数据库错误: {}", e);
            }
        },
    };

    // 创建共享状态
    let state = Arc::new(RwLock::new(State {
        token: None,
        rqbit_session: None,
    }));
    let config = Arc::new(RwLock::new(config));
    let db = Arc::new(RwLock::new(db));

    // 启动数据保存任务
    tokio::spawn(data_save_task(
        db.clone(),
        config.clone(),
        app.config.unwrap_or("./config.json".to_owned()),
    ));

    // 创建消息通道
    let event_task_channel = mpsc::channel(1000);

    // 启动事件处理任务
    tokio::spawn(event_handle_task(
        config.clone(),
        db.clone(),
        event_task_channel.0.clone(),
        state.clone(),
        event_task_channel.1,
    ));

    let download_task_channel = mpsc::channel(1000);
    tokio::spawn(download::download_command_task(
        download_task_channel.1,
        state.clone(),
        config.clone(),
    ));

    // 创建 TCP 监听器
    let sock = TcpListener::new("[::]:8001").bind().await;

    // 创建路由器并添加中间件和路由
    let router = Router::new()
        .hoop(affix_state::inject(event_task_channel.0))
        .hoop(affix_state::inject(download_task_channel.0))
        .hoop(affix_state::inject(config))
        .hoop(affix_state::inject(state))
        .hoop(affix_state::inject(db))
        .push(Router::with_path("/api").append(&mut api::routes()));

    // 创建服务并添加 CORS 中间件
    let service = Service::new(router).hoop(
        Cors::new()
            .allow_origin("*")
            .allow_methods(AllowMethods::any())
            .allow_credentials(AllowCredentials::judge(|_, _, _| true))
            .allow_headers(AllowHeaders::any())
            .into_handler(),
    );

    // 启动服务器
    Server::new(sock).serve(service).await;

    Ok(())
}

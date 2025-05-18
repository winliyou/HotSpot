mod config;
mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;
mod validators;
mod ws;

use axum::{Router, http::Method, routing::get};
use config::app_config::AppConfig;
use config::db::Database;
use dotenv::dotenv;
use middleware::auth::{AppState, auth_middleware};
use routes::{checkin_routes, group_routes, message_routes, user_routes};
use services::{
    CheckinService, ConfigService, GroupService, MessageService, UserService, WsService,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ws::handler::ws_handler;
use ws::session::SessionManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载.env文件
    dotenv().ok();

    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("初始化配置...");
    // 创建应用配置
    let app_config = AppConfig::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "devkey".into()),
        std::env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "3600".into()),
        std::env::var("DATABASE_URL").expect("数据库连接URL未设置"),
        std::env::var("REDIS_URL").ok(),
        std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .unwrap_or(8080),
    );
    let port = app_config.server_port;

    info!("连接数据库...");
    let db = Database::connect(&app_config.database_url, app_config.redis_url.as_deref()).await?;

    // 创建会话管理器
    let session_manager = Arc::new(SessionManager::new());

    // 创建服务
    let config_service = Arc::new(ConfigService::new(app_config.clone()));
    let user_service = Arc::new(UserService::new(db.clone(), config_service.clone()));
    let group_service = Arc::new(GroupService::new(db.clone()));
    let message_service = Arc::new(MessageService::new(db.pg_pool.clone()));
    let checkin_service = Arc::new(CheckinService::new(db.clone()));
    let ws_service = Arc::new(WsService::new(
        session_manager.clone().as_ref().clone(),
        config_service.clone(),
    ));

    // 创建统一的应用状态
    let state = Arc::new(AppState {
        config_service,
        user_service,
        group_service,
        message_service,
        checkin_service,
        session_manager,
        ws_service,
    });

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // 创建带有状态的路由
    let user_routes = user_routes();
    let group_routes = group_routes();
    let message_routes = message_routes();
    let checkin_routes = checkin_routes();
    let ws_route = Router::new().route("/", get(ws_handler));

    // 创建主路由
    let mut app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/v1/user", user_routes)
                .nest("/v1/group", group_routes)
                .nest("/v1/chat", message_routes)
                .nest("/v1/checkin", checkin_routes),
        )
        .nest("/ws", ws_route)
        .layer(cors)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));
    #[cfg(debug_assertions)]
    {
        let cors = CorsLayer::very_permissive();
        app = app.layer(cors);
    }

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("启动服务器，监听端口 {}", port);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.with_state(state)).await?;

    Ok(())
}

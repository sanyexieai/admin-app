use crate::routers::user_router::user_router;
use crate::routers::permission_router::permission_router;
use axum::{Extension, Router};
use helpers::{db_helper, dev_helper::get_dev};
use migration::cli;
use routers::role_router::role_router;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
mod controllers;
mod helpers;
mod middleware;
mod models;
mod routers;
mod server;
#[tokio::main]
async fn main() {
    //数据迁移
    cli::run_cli(migration::Migrator).await;
    let db: Arc<sea_orm::DatabaseConnection> = Arc::new(db_helper::db_connection().await);
    let user_routers = user_router(Extension(db.clone()));
    let premissons_routers = permission_router(Extension(db.clone()));
    let role_routers = role_router(Extension(db.clone()));
    // 创建一个基本的CORS配置
    let cors: CorsLayer = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any)
    .allow_headers(Any);

    let app = Router::new()
        .merge(user_routers)
        .merge(premissons_routers)
        .merge(role_routers)
        .layer(cors)
        .layer(Extension(db));


    let addr = get_dev("ADDR".to_string(), format!("{}",format!("0.0.0.0:3000")).into()).await;
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

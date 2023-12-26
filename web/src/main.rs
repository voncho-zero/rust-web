use std::env;
use std::sync::Arc;
use axum::{Router};
use axum::routing::get;

use dotenvy::dotenv;
use rbatis::RBatis;
use redis::aio::Connection;
use redis::Client;
use redis_pool::RedisPool;

mod base;

struct AppState {
    rbatis: RBatis,
    redis_pool: RedisPool<Client, Connection>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let host = env::var("HOST").expect("服务器地址未配置");
    let port = env::var("PORT").expect("服务器端口未配置");
    let addr = format!("{host}:{port}");
    let db_url = env::var("DB_URL").expect("数据库连接未配置");
    let redis_url = env::var("REDIS_URL").expect("redis连接未配置");
    let client = redis::Client::open(redis_url).unwrap();
    let pool = RedisPool::from(client);
    let rb: RBatis = services::init_db(&db_url).await;
    let shared_state = Arc::new(AppState {
        rbatis: rb,
        redis_pool: pool
    });

    let app = Router::new()
        .nest_service("/base", base::routes(shared_state.clone()))
        .route("/", get(web::index_handler))
        .with_state(shared_state);

    tracing::info!("Server started, listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


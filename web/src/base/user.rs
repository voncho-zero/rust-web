use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::{get};
use tracing::info;
use models::base::User;
use models::Message;
use crate::AppState;
use services::base::user;

pub fn routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/:id", get(get_user))
        .route("/list", get(list_user))
        .route("/create", get(add_user))
        .with_state(state)
}


async fn get_user(State(state): State<Arc<AppState>>, Path(id): Path<usize>) -> Json<Option<User>> {
    info!("{id}");
    let data = User::select_by_id(&state.rbatis, id).await.unwrap();
    Json(data)
}

async fn list_user(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let list = User::select_all(&state.rbatis).await.unwrap();
    let users = user::list_user_by_ids(&state.rbatis, vec![3, 4]).await.unwrap();
    dbg!(users);
    Json(list)
}

async fn add_user(State(state): State<Arc<AppState>>) -> Json<Message> {
    let usr = user::create_user(&state.rbatis).await;
    info!("{}", usr);
    Json(Message {
        message: String::from("添加成功")
    })
}
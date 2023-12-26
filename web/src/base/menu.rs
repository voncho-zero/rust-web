use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use tracing::info;
use models::base::Menu;
use models::TreeNode;
use crate::AppState;

pub fn routes<S>(state: Arc<AppState>) -> Router<S>{
    Router::new()
        .route("/:id", get(get_menu))
        .route("/list", get(list_menu))
        .with_state(state)
}


async fn get_menu(State(s): State<Arc<AppState>>, Path(id): Path<usize>) -> Json<Option<Menu>> {
    info!("{id}");
    let menu = Menu::select_by_id(&s.rbatis, id).await.unwrap();
    Json(menu)
}

async fn list_menu(State(s): State<Arc<AppState>>,) -> Json<Vec<Rc<RefCell<TreeNode<Menu>>>>>{
    let list = Menu::select_all(&s.rbatis).await.unwrap_or_default();
    Json(TreeNode::list_to_tree(list))
}

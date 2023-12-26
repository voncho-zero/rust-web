use axum::Json;
use models::Message;



pub async fn index_handler() -> Json<Message>{
    Json(Message {
        message: String::from("Hello Axum!"),
    })
}
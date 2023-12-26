mod user;
mod menu;

use axum::{
    extract::State,
    Router,
    routing::get,
    response::sse::{
        Event,
        KeepAlive,
        Sse
    },
    Json
};
use std::{sync::Arc, time::Duration, convert::Infallible};
use axum_extra::TypedHeader;
use futures::{stream, Stream};
use rand::prelude::SliceRandom;
use tracing::{info, warn};
use models::Message;
use crate::AppState;
use redis::{AsyncCommands};
use tokio_stream::StreamExt;


pub fn routes<S>(state: Arc<AppState>) -> Router<S> {

    Router::new().route("/", get(handler))
        .route("/lottery", get(lottery))
        .route("/sse", get(sse_handler))
        .nest("/user", user::routes(state.clone()))
        .nest("/menu", menu::routes(state.clone()))
        .with_state(state)
}



pub async fn handler(State(state): State<Arc<AppState>>) -> Json<Message> {
    let count: u64 = state.rbatis
        .query_decode("select count(1) as count from user", vec![])
        .await
        .unwrap();
    info!("user has {count} items");
    Json(Message {
        message: String::from("hello, world!")
    })
}

async fn lottery(State(state): State<Arc<AppState>>) -> Json<Vec<i32>>{
    let mut con = state.redis_pool.aquire().await.unwrap();
    let _: () = con.set("test", "test_data").await.unwrap();
    let rv: String = con.get("test").await.unwrap();
    warn!("{rv}");
    /*let _: () = redis::pipe()
        .set(0, "Hello")
        .ignore()
        .query_async(&mut con)
        .await
        .unwrap();
    let str: String = redis::cmd("GET").arg(0).query_async(&mut con).await.unwrap();
    info!("{str}");*/
    let mut rng = rand::thread_rng();
    let mut arr = vec![1,2,3,4,5,6,7];
    arr.shuffle(&mut rng);
    Json(arr)
}
/*async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let mut i = 0;
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(move || {
        i += 1;
        Event::default().data(format!("hi,{}", &i))
    })
        .map(Ok)
        .throttle(Duration::from_secs(3)); //每3秒，向浏览器发1次消息

    //每隔1秒发1次保活
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}*/

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
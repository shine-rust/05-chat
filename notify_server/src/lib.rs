use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::postgres::PgListener;
use tokio_stream::StreamExt;
use tracing::info;

mod sse;

use sse::sse_handler;

const INDEX_HTML: &str = include_str!("../index.html");

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(sse_handler))
}

pub async fn setup_pg_listener() -> anyhow::Result<()> {
    let mut listener = PgListener::connect("postgres://felix:postgres@localhost:5432/chat").await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;

    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(Ok(notif)) = stream.next().await {
            info!("Received notification: {:?}", notif);
        }
    });
    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}

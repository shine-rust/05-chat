use crate::{AppError, AppState};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Extension, Json};
use chat_core::User;

pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.fetch_chat_users(user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(users)).into_response())
}

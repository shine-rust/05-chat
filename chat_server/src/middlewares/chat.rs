use crate::{AppError, AppState, User};
use axum::extract::{FromRequestParts, Path, Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn verify_chat(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    let Path(chat_id) = Path::<u64>::from_request_parts(&mut parts, &state)
        .await
        .unwrap();

    let user = parts.extensions.get::<User>().unwrap();
    if !state
        .is_chat_member(chat_id, user.id as _)
        .await
        .unwrap_or_default()
    {
        return AppError::CreateMessageError(format!(
            "User {} are not a member of chat {}",
            user.id, chat_id
        ))
        .into_response();
    }
    let req = Request::from_parts(parts, body);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middlewares::verify_token;
    use anyhow::Result;
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::middleware::from_fn_with_state;
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    async fn handler() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }

    #[tokio::test]
    async fn verify_chat_middleware_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let user = state.find_user_by_id(1).await?.expect("user should exist");

        let token = state.ek.sign(user)?;

        let app = Router::new()
            .route("/chat/{id}/message", get(handler))
            .layer(from_fn_with_state(state.clone(), verify_chat))
            .layer(from_fn_with_state(state.clone(), verify_token))
            .with_state(state);

        // user in chat
        let req = Request::builder()
            .uri("/chat/1/message")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // user not in chat
        let req = Request::builder()
            .uri("/chat/5/message")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}

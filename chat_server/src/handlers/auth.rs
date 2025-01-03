use axum::response::IntoResponse;

pub(crate) async fn signin_handler() -> impl IntoResponse {
    "Sign in"
}

pub(crate) async fn signup_handler() -> impl IntoResponse {
    "Sign up"
}

use crate::routers::v1::auth::login_handler;
use axum::{Router, routing::post};

pub fn v1_router() -> Router {
    Router::new().route("/v1/auth/login", post(login_handler))
}

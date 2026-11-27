use axum::{middleware::from_fn_with_state, Router};

pub mod auth;
pub mod blog;
pub mod root;
pub mod user;

use app::state::AppState;
use auth::{create_password_router, create_session_router};
use blog::create_blog_router;
use root::create_root_router;
use user::create_user_router;

use crate::middleware::{limit, RateLimiter};

pub fn create_router(state: AppState, auth_limiter: RateLimiter) -> Router {
    // only where passwords can be guessed; refreshing must not use up that budget
    let auth = create_password_router()
        .route_layer(from_fn_with_state(auth_limiter, limit))
        .merge(create_session_router());

    Router::new()
        .merge(create_root_router())
        .nest("/auth", auth)
        .nest("/users", create_user_router())
        .nest("/blogs", create_blog_router())
        .with_state(state)
}

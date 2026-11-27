use axum::{middleware::from_fn_with_state, Router};

pub mod auth;
pub mod blog;
pub mod root;
pub mod user;

use app::state::AppState;
use auth::create_auth_router;
use blog::create_blog_router;
use root::create_root_router;
use user::create_user_router;

use crate::middleware::{limit, RateLimiter};

pub fn create_router(state: AppState, auth_limiter: RateLimiter) -> Router {
    // only the unauthenticated endpoints, where passwords can be guessed
    let auth = create_auth_router().route_layer(from_fn_with_state(auth_limiter, limit));

    Router::new()
        .merge(create_root_router())
        .nest("/auth", auth)
        .nest("/users", create_user_router())
        .nest("/blogs", create_blog_router())
        .with_state(state)
}

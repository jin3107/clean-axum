use api::setup_router;
use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use utils::testing::{make_request, setup_test_db};

mod auth;
mod blog;
mod root;
mod tenancy;

pub(super) const TEST_SECRET: &str = "test-only-secret-0123456789abcdef";

pub(super) async fn app() -> Router {
    let db = setup_test_db().await.expect("Set up db failed!");
    setup_router(db, TEST_SECRET)
}

/// Tenant names are unique per run so tests can share a real database.
pub(super) fn unique(prefix: &str) -> String {
    format!("{prefix}-{}", uuid::Uuid::new_v4())
}

pub(super) async fn call(
    app: &Router,
    method: &str,
    url: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (u16, Value) {
    let body = body.map(|body| body.to_string());
    let response = make_request(app.clone(), method, url, token, body).await;
    let status = response.status().as_u16();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// Registers a new tenant and returns the admin's token response.
pub(super) async fn register(app: &Router, tenant: &str) -> Value {
    let body = json!({"tenant_name": tenant, "username": "admin", "password": "password-123"});
    let (status, body) = call(app, "POST", "/auth/register", None, Some(body)).await;
    assert_eq!(status, 201, "{body}");
    body
}

pub(super) fn token(auth: &Value) -> &str {
    auth["access_token"].as_str().unwrap()
}

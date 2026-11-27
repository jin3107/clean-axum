use app::auth::JwtKeys;
use axum::{body::Body, http::Request};
use models::domains::role::Role;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use super::{app, app_with, call, security_config, unique, TEST_SECRET};

#[tokio::test]
async fn token_must_match_issuer_and_audience() {
    let app = app().await;
    let token = |issuer: &str, audience: &str| {
        JwtKeys::new(TEST_SECRET, issuer, audience)
            .issue(Uuid::new_v4(), Uuid::new_v4(), Role::Admin)
            .unwrap()
    };

    for (issuer, audience, expected) in [
        ("test-issuer", "test-audience", 200),
        ("other-issuer", "test-audience", 401),
        ("test-issuer", "other-audience", 401),
    ] {
        let token = token(issuer, audience);
        let (status, _) = call(&app, "GET", "/users", Some(&token), None).await;
        assert_eq!(status, expected, "{issuer} {audience}");
    }
}

#[tokio::test]
async fn auth_endpoints_are_rate_limited() {
    let mut security = security_config();
    security.auth_rate_limit = 3;
    let app = app_with(security).await;
    let login =
        json!({"tenant_name": unique("nobody"), "username": "admin", "password": "wrong-one"});

    for _ in 0..3 {
        let (status, _) = call(&app, "POST", "/auth/login", None, Some(login.clone())).await;
        assert_eq!(status, 401);
    }
    let (status, body) = call(&app, "POST", "/auth/login", None, Some(login.clone())).await;
    assert_eq!(status, 429);
    assert_eq!(body["message"], "Too many requests");
    // login and register share the budget
    let (status, _) = call(&app, "POST", "/auth/register", None, Some(login)).await;
    assert_eq!(status, 429);
    // other routes are not affected
    let (status, _) = call(&app, "GET", "/users", None, None).await;
    assert_eq!(status, 401);
}

#[tokio::test]
async fn only_listed_origins_pass_cors() {
    let app = app().await;
    for (origin, allowed) in [
        ("http://localhost:5173", true),
        ("http://evil.example", false),
    ] {
        let preflight = Request::builder()
            .method("OPTIONS")
            .uri("/users")
            .header("Origin", origin)
            .header("Access-Control-Request-Method", "GET")
            .header("Access-Control-Request-Headers", "authorization")
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(preflight).await.unwrap();
        let header = response.headers().get("access-control-allow-origin");
        assert_eq!(header.is_some(), allowed, "{origin}");
        if allowed {
            assert_eq!(header.unwrap(), origin);
        }
    }
}

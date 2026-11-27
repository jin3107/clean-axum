use axum::Router;
use serde_json::{json, Value};

use super::{app, app_with, call, register, security_config, token, unique};

async fn post(app: &Router, url: &str, refresh_token: &str) -> (u16, Value) {
    let body = json!({"refresh_token": refresh_token});
    call(app, "POST", url, None, Some(body)).await
}

fn refresh_token(auth: &Value) -> &str {
    auth["refresh_token"].as_str().unwrap()
}

#[tokio::test]
async fn refresh_rotates_the_token() {
    let app = app().await;
    let auth = register(&app, &unique("clinic")).await;
    assert_eq!(auth["expires_in"], 900);

    let (status, next) = post(&app, "/auth/refresh", refresh_token(&auth)).await;
    assert_eq!(status, 200, "{next}");
    assert_ne!(refresh_token(&next), refresh_token(&auth));
    assert_eq!(next["user"]["id"], auth["user"]["id"]);

    let (status, _) = call(&app, "GET", "/users", Some(token(&next)), None).await;
    assert_eq!(status, 200);
    let (status, _) = post(&app, "/auth/refresh", refresh_token(&next)).await;
    assert_eq!(status, 200);
}

#[tokio::test]
async fn reusing_a_token_ends_the_whole_session() {
    let app = app().await;
    let tenant = unique("clinic");
    let auth = register(&app, &tenant).await;
    // a second login is a separate session
    let login = json!({"tenant_name": tenant, "username": "admin", "password": "password-123"});
    let (_, other) = call(&app, "POST", "/auth/login", None, Some(login)).await;

    let (_, next) = post(&app, "/auth/refresh", refresh_token(&auth)).await;
    let (status, _) = post(&app, "/auth/refresh", refresh_token(&auth)).await;
    assert_eq!(status, 401);
    let (status, _) = post(&app, "/auth/refresh", refresh_token(&next)).await;
    assert_eq!(status, 401, "the newer token must be revoked too");

    let (status, _) = post(&app, "/auth/refresh", refresh_token(&other)).await;
    assert_eq!(status, 200);
}

#[tokio::test]
async fn concurrent_refresh_succeeds_at_most_once() {
    let app = app().await;
    let auth = register(&app, &unique("clinic")).await;
    let token = refresh_token(&auth);

    let (a, b) = tokio::join!(
        post(&app, "/auth/refresh", token),
        post(&app, "/auth/refresh", token)
    );
    let mut statuses = [a.0, b.0];
    statuses.sort_unstable();
    assert_eq!(statuses, [200, 401]);
}

#[tokio::test]
async fn logout_revokes_the_session() {
    let app = app().await;
    let auth = register(&app, &unique("clinic")).await;

    for _ in 0..2 {
        let (status, _) = post(&app, "/auth/logout", refresh_token(&auth)).await;
        assert_eq!(status, 204);
    }
    let (status, _) = post(&app, "/auth/refresh", refresh_token(&auth)).await;
    assert_eq!(status, 401);
}

#[tokio::test]
async fn unknown_and_cross_tenant_tokens_are_rejected() {
    let app = app().await;
    let a = register(&app, &unique("clinic-a")).await;
    let b = register(&app, &unique("clinic-b")).await;
    let secret = |auth: &Value| refresh_token(auth).split_once('.').unwrap().1.to_string();
    let tenant = |auth: &Value| refresh_token(auth).split_once('.').unwrap().0.to_string();
    let moved = format!("{}.{}", tenant(&b), secret(&a));

    for bad in ["garbage", "not-a-uuid.abc", moved.as_str()] {
        let (status, _) = post(&app, "/auth/refresh", bad).await;
        assert_eq!(status, 401, "{bad}");
    }
    // the failed attempts must not have damaged the real sessions
    for auth in [&a, &b] {
        let (status, _) = post(&app, "/auth/refresh", refresh_token(auth)).await;
        assert_eq!(status, 200);
    }
}

#[tokio::test]
async fn refreshing_is_not_rate_limited_and_input_is_not_echoed() {
    let mut security = security_config();
    security.auth_rate_limit = 1;
    let app = app_with(security).await;
    let auth = register(&app, &unique("clinic")).await;

    let mut current = auth;
    for _ in 0..3 {
        let (status, next) = post(&app, "/auth/refresh", refresh_token(&current)).await;
        assert_eq!(status, 200);
        current = next;
    }
    let (status, body) = post(&app, "/auth/refresh", &"s3cret".repeat(100)).await;
    assert_eq!(status, 422);
    assert!(!body.to_string().contains("s3cret"), "{body}");
}

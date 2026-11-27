use serde_json::json;

use super::{app, call, other_role, register, token, unique, TEST_SECRET};

#[tokio::test]
async fn register_then_login() {
    let app = app().await;
    let tenant = unique("clinic");
    let auth = register(&app, &tenant).await;
    assert_eq!(auth["user"]["role"], "admin");
    assert!(auth["user"].get("password_hash").is_none());

    let login = |tenant: String, password: &str| json!({"tenant_name": tenant, "username": "admin", "password": password});
    let (status, body) = call(
        &app,
        "POST",
        "/auth/login",
        None,
        Some(login(tenant.clone(), "password-123")),
    )
    .await;
    assert_eq!(status, 200);
    assert_eq!(body["user"]["id"], auth["user"]["id"]);

    for body in [
        login(tenant.clone(), "wrong-password"),
        login(unique("nobody"), "password-123"),
    ] {
        let (status, body) = call(&app, "POST", "/auth/login", None, Some(body)).await;
        assert_eq!(status, 401);
        assert_eq!(body["message"], "Invalid credentials");
    }
}

#[tokio::test]
async fn duplicate_tenant_is_conflict() {
    let app = app().await;
    let tenant = unique("clinic");
    register(&app, &tenant).await;

    let body = json!({"tenant_name": tenant, "username": "other", "password": "password-123"});
    let (status, body) = call(&app, "POST", "/auth/register", None, Some(body)).await;
    assert_eq!(status, 409);
    assert_eq!(body["message"], "Resource already exists");
}

#[tokio::test]
async fn rejected_password_is_not_echoed() {
    let app = app().await;
    let body = json!({"tenant_name": unique("clinic"), "username": "admin", "password": "s3cret!"});
    let (status, body) = call(&app, "POST", "/auth/register", None, Some(body)).await;
    assert_eq!(status, 422);
    assert_eq!(body["details"]["password"][0]["code"], "length");
    assert!(!body.to_string().contains("s3cret!"), "{body}");
}

#[tokio::test]
async fn protected_routes_need_a_valid_token() {
    let app = app().await;
    let auth = register(&app, &unique("clinic")).await;

    let forged = app::auth::JwtKeys::new(
        "another-secret-0123456789abcdefgh",
        "test-issuer",
        "test-audience",
    )
    .issue(
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
        models::domains::role::Role::Admin,
    )
    .unwrap();
    assert_ne!(TEST_SECRET, "another-secret-0123456789abcdefgh");

    for bad in [None, Some("garbage"), Some(forged.as_str())] {
        for url in ["/users", "/blogs"] {
            let (status, _) = call(&app, "GET", url, bad, None).await;
            assert_eq!(status, 401, "{url} {bad:?}");
        }
    }
    let (status, _) = call(&app, "GET", "/users", Some(token(&auth)), None).await;
    assert_eq!(status, 200);
}

#[tokio::test]
async fn only_admin_creates_users() {
    let app = app().await;
    let tenant = unique("clinic");
    let admin = register(&app, &tenant).await;
    let role = other_role();
    let member = json!({"username": "member", "password": "password-123", "role": role});

    let (status, body) = call(
        &app,
        "POST",
        "/users",
        Some(token(&admin)),
        Some(member.clone()),
    )
    .await;
    assert_eq!(status, 201, "{body}");
    assert_eq!(body["tenant_id"], admin["user"]["tenant_id"]);

    let (status, _) = call(&app, "POST", "/users", Some(token(&admin)), Some(member)).await;
    assert_eq!(status, 409);

    let login = json!({"tenant_name": tenant, "username": "member", "password": "password-123"});
    let (_, member) = call(&app, "POST", "/auth/login", None, Some(login)).await;
    let escalate = json!({"username": "evil", "password": "password-123", "role": "admin"});
    let (status, _) = call(&app, "POST", "/users", Some(token(&member)), Some(escalate)).await;
    assert_eq!(status, 403);

    let unknown = json!({"username": "x1", "password": "password-123", "role": "no-such-role"});
    let (status, _) = call(&app, "POST", "/users", Some(token(&admin)), Some(unknown)).await;
    assert!(status == 400 || status == 422, "{status}");
}

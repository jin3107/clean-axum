use axum::{body::Body, http::Request, response::Response, Router};
use tower::ServiceExt;

/// Sends one request, optionally with a bearer token and a JSON body.
pub async fn make_request(
    app: Router,
    method: &str,
    url: &str,
    token: Option<&str>,
    body: Option<String>,
) -> Response {
    let mut request = Request::builder().method(method).uri(url);
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    if body.is_some() {
        request = request.header("Content-Type", "application/json");
    }
    let body = body.map_or_else(Body::empty, Body::from);

    app.oneshot(request.body(body).unwrap()).await.unwrap()
}

pub async fn make_get_request(app: Router, url: &str) -> Response {
    make_request(app, "GET", url, None, None).await
}

pub async fn make_post_request(app: Router, url: &str, body: String) -> Response {
    make_request(app, "POST", url, None, Some(body)).await
}

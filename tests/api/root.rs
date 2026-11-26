use axum::http::StatusCode;
use http_body_util::BodyExt;

use utils::testing::make_get_request;

use super::app;

#[tokio::test]
async fn root_is_public() {
    let response = make_get_request(app("root").await, "/").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World from DB!");
}

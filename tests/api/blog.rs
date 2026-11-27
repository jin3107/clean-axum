use serde_json::json;

use super::{app, call, register, token, unique};

#[tokio::test]
async fn author_comes_from_the_token() {
    let app = app().await;
    let admin = register(&app, &unique("clinic")).await;

    // a client-supplied author or tenant is ignored
    let body = json!({
        "title": "title", "content": "test",
        "author_id": uuid::Uuid::new_v4(), "tenant_id": uuid::Uuid::new_v4()
    });
    let (status, created) = call(&app, "POST", "/blogs", Some(token(&admin)), Some(body)).await;
    assert_eq!(status, 201, "{created}");
    assert_eq!(created["author_id"], admin["user"]["id"]);
    assert_eq!(created["tenant_id"], admin["user"]["tenant_id"]);

    let (status, list) = call(&app, "GET", "/blogs?title=tit", Some(token(&admin)), None).await;
    assert_eq!(status, 200);
    assert_eq!(list["blogs"], json!([created]));
}

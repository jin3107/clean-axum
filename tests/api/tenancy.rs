use serde_json::json;

use super::{app, call, register, token, unique};

#[tokio::test]
async fn tenants_cannot_see_each_other() {
    let app = app().await;
    let a = register(&app, &unique("clinic-a")).await;
    // the same username is allowed in another tenant
    let b = register(&app, &unique("clinic-b")).await;

    let blog = json!({"title": "private", "content": "tenant a only"});
    let (status, _) = call(&app, "POST", "/blogs", Some(token(&a)), Some(blog)).await;
    assert_eq!(status, 201);

    let (_, blogs) = call(&app, "GET", "/blogs", Some(token(&b)), None).await;
    assert_eq!(blogs["blogs"], json!([]));

    let (_, users) = call(&app, "GET", "/users", Some(token(&b)), None).await;
    assert_eq!(users["users"].as_array().unwrap().len(), 1);
    assert_eq!(users["users"][0]["id"], b["user"]["id"]);

    let url = format!("/users/{}", a["user"]["id"].as_str().unwrap());
    let (status, _) = call(&app, "GET", &url, Some(token(&b)), None).await;
    assert_eq!(status, 404);
    let (status, _) = call(&app, "GET", &url, Some(token(&a)), None).await;
    assert_eq!(status, 200);
}

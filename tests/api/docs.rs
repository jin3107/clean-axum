use axum::Router;
use doc::ApiDoc;
use http_body_util::BodyExt;
use serde_json::Value;
use utils::testing::make_get_request;

/// Every `$ref` anywhere inside `value`.
fn references<'a>(value: &'a Value, found: &mut Vec<&'a str>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                match child.as_str() {
                    Some(target) if key == "$ref" => found.push(target),
                    _ => references(child, found),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|item| references(item, found)),
        _ => {}
    }
}

#[tokio::test]
async fn every_reference_in_the_api_document_resolves() {
    let response = make_get_request(Router::new().attach_doc(), "/openapi.json").await;
    assert_eq!(response.status(), 200);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let document: Value = serde_json::from_slice(&bytes).unwrap();

    let mut found = Vec::new();
    references(&document, &mut found);
    assert!(!found.is_empty());

    for target in found {
        let pointer = target.strip_prefix('#').expect("a local reference");
        assert!(document.pointer(pointer).is_some(), "{target} is missing");
    }
}

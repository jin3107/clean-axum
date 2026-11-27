use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Allows only the listed origins. With an empty list no CORS header is sent, so
/// browsers keep other origins out. Panics at startup on an unusable value.
pub fn cors_layer(origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = origins
        .iter()
        .map(|origin| {
            assert!(origin != "*", "CORS_ORIGINS must list origins, not *");
            origin
                .parse()
                .unwrap_or_else(|_| panic!("CORS_ORIGINS has an invalid origin: {origin}"))
        })
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
}

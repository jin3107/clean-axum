use axum::Router;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod blog;
mod root;
mod user;

struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                "bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = root::RootApi),
        (path = "/auth", api = auth::AuthApi),
        (path = "/users", api = user::UserApi),
        (path = "/blogs", api = blog::BlogApi),
    ),
    modifiers(&BearerAuth),
    tags(
        (name = "root", description = "Root API"),
        (name = "auth", description = "Authentication API"),
        (name = "user", description = "User API"),
        (name = "blog", description = "Blog API"),

    )
)]
struct _ApiDoc;

pub trait ApiDoc {
    fn attach_doc(self) -> Self;
}

impl ApiDoc for Router {
    fn attach_doc(self) -> Self {
        self.merge(SwaggerUi::new("/docs").url("/openapi.json", _ApiDoc::openapi()))
            .merge(Scalar::with_url("/scalar", _ApiDoc::openapi()))
    }
}

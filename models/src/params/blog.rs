use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

// The author and tenant come from the access token, never from the client.
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateBlogParams {
    #[validate(length(min = 2, max = 255))]
    pub title: String,

    #[validate(length(min = 2, max = 255))]
    pub content: String,
}

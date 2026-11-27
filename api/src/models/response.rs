use std::collections::HashMap;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationErrorResponse<T> {
    pub message: String,
    /// The problems found, keyed by field name.
    // described by hand: a map used as a generic argument is referenced as a
    // schema that never gets registered
    #[schema(value_type = std::collections::HashMap<String, Vec<Object>>)]
    pub details: T,
}

pub type ParamsErrorResponse =
    ValidationErrorResponse<HashMap<String, Vec<HashMap<String, String>>>>;

impl<T> From<T> for ValidationErrorResponse<T> {
    fn from(t: T) -> Self {
        Self {
            message: "Validation error".to_string(),
            details: t,
        }
    }
}

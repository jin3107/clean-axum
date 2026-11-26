use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domains::blog;

#[derive(Serialize, ToSchema)]
pub struct BlogSchema {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
}

impl From<blog::Model> for BlogSchema {
    fn from(blog: blog::Model) -> Self {
        Self {
            id: blog.id,
            tenant_id: blog.tenant_id,
            title: blog.title,
            content: blog.content,
            author_id: blog.author_id,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct BlogListSchema {
    pub blogs: Vec<BlogSchema>,
}

impl From<Vec<blog::Model>> for BlogListSchema {
    fn from(blogs: Vec<blog::Model>) -> Self {
        Self {
            blogs: blogs.into_iter().map(BlogSchema::from).collect(),
        }
    }
}

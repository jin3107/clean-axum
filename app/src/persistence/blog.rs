use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use models::{domains::blog, params::blog::CreateBlogParams, queries::blog::BlogQuery};

use crate::tenant::TenantId;

pub async fn search_blogs(
    db: &DbConn,
    tenant: TenantId,
    query: BlogQuery,
) -> Result<Vec<blog::Model>, DbErr> {
    blog::Entity::find()
        .filter(blog::Column::TenantId.eq(tenant.0))
        .filter(blog::Column::Title.contains(query.title.unwrap_or_default()))
        .all(db)
        .await
}

pub async fn create_blog(
    db: &DbConn,
    tenant: TenantId,
    author_id: Uuid,
    params: CreateBlogParams,
) -> Result<blog::Model, DbErr> {
    blog::ActiveModel {
        tenant_id: Set(tenant.0),
        id: Set(Uuid::new_v4()),
        author_id: Set(author_id),
        title: Set(params.title),
        content: Set(params.content),
    }
    .insert(db)
    .await
}

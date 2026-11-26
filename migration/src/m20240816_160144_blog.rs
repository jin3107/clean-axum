use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_init::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blog::Table)
                    .if_not_exists()
                    .col(uuid(Blog::TenantId))
                    .col(uuid(Blog::Id))
                    .col(uuid(Blog::AuthorId))
                    .col(string(Blog::Title))
                    .col(string(Blog::Content))
                    .primary_key(Index::create().col(Blog::TenantId).col(Blog::Id))
                    // composite key: an author must belong to the same tenant
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_blog_author")
                            .from(Blog::Table, (Blog::TenantId, Blog::AuthorId))
                            .to(User::Table, (User::TenantId, User::Id))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blog {
    Table,
    TenantId,
    Id,
    AuthorId,
    Title,
    Content,
}

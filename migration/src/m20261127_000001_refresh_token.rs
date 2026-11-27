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
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(uuid(RefreshToken::TenantId))
                    .col(uuid(RefreshToken::Id))
                    .col(uuid(RefreshToken::UserId))
                    .col(uuid(RefreshToken::FamilyId))
                    .col(string_len(RefreshToken::TokenHash, 64))
                    // unix seconds
                    .col(big_integer(RefreshToken::ExpiresAt))
                    .col(big_integer_null(RefreshToken::RevokedAt))
                    .primary_key(
                        Index::create()
                            .col(RefreshToken::TenantId)
                            .col(RefreshToken::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refresh_token_user")
                            .from(
                                RefreshToken::Table,
                                (RefreshToken::TenantId, RefreshToken::UserId),
                            )
                            .to(User::Table, (User::TenantId, User::Id))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("ux_refresh_token_tenant_hash")
                    .table(RefreshToken::Table)
                    .col(RefreshToken::TenantId)
                    .col(RefreshToken::TokenHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("ix_refresh_token_tenant_family")
                    .table(RefreshToken::Table)
                    .col(RefreshToken::TenantId)
                    .col(RefreshToken::FamilyId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RefreshToken {
    Table,
    TenantId,
    Id,
    UserId,
    FamilyId,
    TokenHash,
    ExpiresAt,
    RevokedAt,
}

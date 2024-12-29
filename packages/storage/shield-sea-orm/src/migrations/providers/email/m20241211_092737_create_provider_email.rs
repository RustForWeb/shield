use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::base::BaseTable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                BaseTable::create(EmailAuthToken::Table, manager)
                    .col(
                        ColumnDef::new(EmailAuthToken::Email)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailAuthToken::Token)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailAuthToken::ExpiredAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name(EmailAuthToken::UniqueEmailToken.to_string())
                            .col(EmailAuthToken::Email)
                            .col(EmailAuthToken::Token)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailAuthToken::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EmailAuthToken {
    Table,

    Email,
    Token,
    ExpiredAt,

    UniqueEmailToken,
}

use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .add_column(ColumnDef::new(OauthProvider::IconUrl).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::IconUrl)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OauthProvider {
    Table,

    IconUrl,
}

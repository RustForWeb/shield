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
                    .table(OidcProvider::Table)
                    .add_column(ColumnDef::new(OidcProvider::IconUrl).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OidcProvider::Table)
                    .drop_column(OidcProvider::IconUrl)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OidcProvider {
    Table,

    IconUrl,
}

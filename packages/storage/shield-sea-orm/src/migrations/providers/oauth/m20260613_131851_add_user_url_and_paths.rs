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
                    .add_column(ColumnDef::new(OauthProvider::UserUrl).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .add_column(ColumnDef::new(OauthProvider::UserPath).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .add_column(ColumnDef::new(OauthProvider::UserIdPath).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .add_column(ColumnDef::new(OauthProvider::UserEmailPath).text())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .add_column(ColumnDef::new(OauthProvider::UserNamePath).text())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::UserNamePath)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::UserEmailPath)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::UserIdPath)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::UserPath)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OauthProvider::Table)
                    .drop_column(OauthProvider::UserUrl)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OauthProvider {
    Table,

    UserUrl,
    UserPath,
    UserIdPath,
    UserEmailPath,
    UserNamePath,
}

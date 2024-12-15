use async_trait::async_trait;
use sea_orm::DatabaseBackend;
use sea_orm_migration::prelude::{extension::postgres::Type, *};

use crate::base::{Base, BaseTable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DatabaseBackend::MySql | DatabaseBackend::Sqlite => {}
            DatabaseBackend::Postgres => {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(OauthProviderType::Table)
                            .values(OauthProviderType::variants())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        Type::create()
                            .as_enum(OauthProviderVisibility::Table)
                            .values(OauthProviderVisibility::variants())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        Type::create()
                            .as_enum(OauthProviderPkceCodeChallenge::Table)
                            .values(OauthProviderPkceCodeChallenge::variants())
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                BaseTable::create(OauthProvider::Table, manager)
                    .col(
                        ColumnDef::new(OauthProvider::Name)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthProvider::Slug).string_len(256))
                    .col({
                        let mut column = ColumnDef::new(OauthProvider::Type)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(
                                    OauthProviderType::Table,
                                    OauthProviderType::variants(),
                                )
                                .into_column_def(),
                            DatabaseBackend::Postgres => {
                                column.custom(OauthProviderType::Table).into_column_def()
                            }
                        }
                    })
                    .col({
                        let mut column = ColumnDef::new(OauthProvider::Visibility)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(
                                    OauthProviderVisibility::Table,
                                    OauthProviderVisibility::variants(),
                                )
                                .into_column_def(),
                            DatabaseBackend::Postgres => column
                                .custom(OauthProviderVisibility::Table)
                                .into_column_def(),
                        }
                    })
                    .col(ColumnDef::new(OauthProvider::ClientId).text().not_null())
                    .col(ColumnDef::new(OauthProvider::ClientSecret).text())
                    .col(ColumnDef::new(OauthProvider::Scopes).text())
                    .col(ColumnDef::new(OauthProvider::RedirectUrl).text())
                    .col(ColumnDef::new(OauthProvider::AuthorizationUrl).text())
                    .col(ColumnDef::new(OauthProvider::AuthorizationUrlParams).text())
                    .col(ColumnDef::new(OauthProvider::TokenUrl).text())
                    .col(ColumnDef::new(OauthProvider::TokenUrlParams).text())
                    .col(ColumnDef::new(OauthProvider::IntrospectionUrl).text())
                    .col(ColumnDef::new(OauthProvider::IntrospectionUrlParams).text())
                    .col(ColumnDef::new(OauthProvider::RevocationUrl).text())
                    .col(ColumnDef::new(OauthProvider::RevocationUrlParams).text())
                    .col({
                        let mut column = ColumnDef::new(OauthProvider::PkceCodeChallenge)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(
                                    OauthProviderPkceCodeChallenge::Table,
                                    OauthProviderPkceCodeChallenge::variants(),
                                )
                                .into_column_def(),
                            DatabaseBackend::Postgres => column
                                .custom(OauthProviderPkceCodeChallenge::Table)
                                .into_column_def(),
                        }
                    })
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                BaseTable::create(OauthProviderConnection::Table, manager)
                    .col(
                        ColumnDef::new(OauthProviderConnection::Identifier)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConnection::TokenType)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConnection::AccessToken)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthProviderConnection::RefreshToken).text())
                    .col(ColumnDef::new(OauthProviderConnection::ExpiredAt).timestamp())
                    .col(ColumnDef::new(OauthProviderConnection::Scopes).text())
                    .col(
                        ColumnDef::new(OauthProviderConnection::ProviderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviderConnection::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                OauthProviderConnection::FkOauthProviderConnectionProvider
                                    .to_string(),
                            )
                            .from(
                                OauthProviderConnection::Table,
                                OauthProviderConnection::ProviderId,
                            )
                            .to(OauthProvider::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                OauthProviderConnection::FkOauthProviderConnectionUser.to_string(),
                            )
                            .from(
                                OauthProviderConnection::Table,
                                OauthProviderConnection::UserId,
                            )
                            .to(User::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name(
                                OauthProviderConnection::UniqueOauthProviderIdentifier.to_string(),
                            )
                            .col(OauthProviderConnection::ProviderId)
                            .col(OauthProviderConnection::Identifier)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OauthProviderConnection::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(OauthProvider::Table).to_owned())
            .await?;

        match manager.get_database_backend() {
            DatabaseBackend::MySql | DatabaseBackend::Sqlite => {}
            DatabaseBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(OauthProviderVisibility::Table).to_owned())
                    .await?;

                manager
                    .drop_type(Type::drop().name(OauthProviderType::Table).to_owned())
                    .await?;
            }
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
}

#[derive(DeriveIden)]
enum OauthProviderType {
    Table,

    Custom,
}

impl OauthProviderType {
    fn variants() -> Vec<Self> {
        vec![Self::Custom]
    }
}

#[derive(DeriveIden)]
enum OauthProviderVisibility {
    Table,

    Public,
    Private,
}

impl OauthProviderVisibility {
    fn variants() -> Vec<Self> {
        vec![Self::Public, Self::Private]
    }
}

#[derive(DeriveIden)]
enum OauthProviderPkceCodeChallenge {
    Table,

    None,
    Plain,
    S256,
}

impl OauthProviderPkceCodeChallenge {
    fn variants() -> Vec<Self> {
        vec![Self::None, Self::Plain, Self::S256]
    }
}

#[derive(DeriveIden)]
enum OauthProvider {
    Table,

    Name,
    Slug,
    Type,
    Visibility,
    ClientId,
    ClientSecret,
    Scopes,
    RedirectUrl,
    AuthorizationUrl,
    AuthorizationUrlParams,
    TokenUrl,
    TokenUrlParams,
    IntrospectionUrl,
    IntrospectionUrlParams,
    RevocationUrl,
    RevocationUrlParams,
    PkceCodeChallenge,
}

#[derive(DeriveIden)]
enum OauthProviderConnection {
    Table,

    Identifier,
    TokenType,
    AccessToken,
    RefreshToken,
    ExpiredAt,
    Scopes,

    ProviderId,
    UserId,

    FkOauthProviderConnectionProvider,
    FkOauthProviderConnectionUser,

    UniqueOauthProviderIdentifier,
}

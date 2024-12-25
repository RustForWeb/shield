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
                            .as_enum(OidcProviderType::Table)
                            .values(OidcProviderType::variants())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        Type::create()
                            .as_enum(OidcProviderVisibility::Table)
                            .values(OidcProviderVisibility::variants())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        Type::create()
                            .as_enum(OidcProviderPkceCodeChallenge::Table)
                            .values(OidcProviderPkceCodeChallenge::variants())
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                BaseTable::create(OidcProvider::Table, manager)
                    .col(
                        ColumnDef::new(OidcProvider::Name)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(ColumnDef::new(OidcProvider::Slug).string_len(256))
                    .col({
                        let mut column = ColumnDef::new(OidcProvider::Type)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(OidcProviderType::Table, OidcProviderType::variants())
                                .into_column_def(),
                            DatabaseBackend::Postgres => {
                                column.custom(OidcProviderType::Table).into_column_def()
                            }
                        }
                    })
                    .col({
                        let mut column = ColumnDef::new(OidcProvider::Visibility)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(
                                    OidcProviderVisibility::Table,
                                    OidcProviderVisibility::variants(),
                                )
                                .into_column_def(),
                            DatabaseBackend::Postgres => column
                                .custom(OidcProviderVisibility::Table)
                                .into_column_def(),
                        }
                    })
                    .col(ColumnDef::new(OidcProvider::ClientId).text().not_null())
                    .col(ColumnDef::new(OidcProvider::ClientSecret).text())
                    .col(ColumnDef::new(OidcProvider::Scopes).text())
                    .col(ColumnDef::new(OidcProvider::RedirectUrl).text())
                    .col(ColumnDef::new(OidcProvider::DiscoveryUrl).text())
                    .col(ColumnDef::new(OidcProvider::IssuerUrl).text())
                    .col(ColumnDef::new(OidcProvider::AuthorizationUrl).text())
                    .col(ColumnDef::new(OidcProvider::AuthorizationUrlParams).text())
                    .col(ColumnDef::new(OidcProvider::TokenUrl).text())
                    .col(ColumnDef::new(OidcProvider::TokenUrlParams).text())
                    .col(ColumnDef::new(OidcProvider::IntrospectionUrl).text())
                    .col(ColumnDef::new(OidcProvider::IntrospectionUrlParams).text())
                    .col(ColumnDef::new(OidcProvider::RevocationUrl).text())
                    .col(ColumnDef::new(OidcProvider::RevocationUrlParams).text())
                    .col(ColumnDef::new(OidcProvider::UserInfoUrl).text())
                    .col(ColumnDef::new(OidcProvider::JsonWebKeySetUrl).text())
                    .col(ColumnDef::new(OidcProvider::JsonWebKeySet).json_binary())
                    .col({
                        let mut column = ColumnDef::new(OidcProvider::PkceCodeChallenge)
                            .not_null()
                            .into_column_def();

                        match manager.get_database_backend() {
                            DatabaseBackend::MySql | DatabaseBackend::Sqlite => column
                                .enumeration(
                                    OidcProviderPkceCodeChallenge::Table,
                                    OidcProviderPkceCodeChallenge::variants(),
                                )
                                .into_column_def(),
                            DatabaseBackend::Postgres => column
                                .custom(OidcProviderPkceCodeChallenge::Table)
                                .into_column_def(),
                        }
                    })
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                BaseTable::create(OidcProviderConnection::Table, manager)
                    .col(
                        ColumnDef::new(OidcProviderConnection::Identifier)
                            .string_len(256)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OidcProviderConnection::TokenType)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OidcProviderConnection::AccessToken)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OidcProviderConnection::RefreshToken).text())
                    .col(ColumnDef::new(OidcProviderConnection::IdToken).text())
                    .col(ColumnDef::new(OidcProviderConnection::ExpiredAt).timestamp())
                    .col(ColumnDef::new(OidcProviderConnection::Scopes).text())
                    .col(
                        ColumnDef::new(OidcProviderConnection::ProviderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OidcProviderConnection::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                OidcProviderConnection::FkOidcProviderConnectionProvider
                                    .to_string(),
                            )
                            .from(
                                OidcProviderConnection::Table,
                                OidcProviderConnection::ProviderId,
                            )
                            .to(OidcProvider::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(OidcProviderConnection::FkOidcProviderConnectionUser.to_string())
                            .from(
                                OidcProviderConnection::Table,
                                OidcProviderConnection::UserId,
                            )
                            .to(User::Table, Base::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name(OidcProviderConnection::UniqueOidcProviderIdentifier.to_string())
                            .col(OidcProviderConnection::ProviderId)
                            .col(OidcProviderConnection::Identifier)
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
                    .table(OidcProviderConnection::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(OidcProvider::Table).to_owned())
            .await?;

        match manager.get_database_backend() {
            DatabaseBackend::MySql | DatabaseBackend::Sqlite => {}
            DatabaseBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(OidcProviderVisibility::Table).to_owned())
                    .await?;

                manager
                    .drop_type(Type::drop().name(OidcProviderType::Table).to_owned())
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
enum OidcProviderType {
    Table,

    Custom,
}

impl OidcProviderType {
    fn variants() -> Vec<Self> {
        vec![Self::Custom]
    }
}

#[derive(DeriveIden)]
enum OidcProviderVisibility {
    Table,

    Public,
    Unlisted,
}

impl OidcProviderVisibility {
    fn variants() -> Vec<Self> {
        vec![Self::Public, Self::Unlisted]
    }
}

#[derive(DeriveIden)]
enum OidcProviderPkceCodeChallenge {
    Table,

    None,
    Plain,
    S256,
}

impl OidcProviderPkceCodeChallenge {
    fn variants() -> Vec<Self> {
        vec![Self::None, Self::Plain, Self::S256]
    }
}

#[derive(DeriveIden)]
enum OidcProvider {
    Table,

    Name,
    Slug,
    Type,
    Visibility,
    ClientId,
    ClientSecret,
    Scopes,
    RedirectUrl,
    DiscoveryUrl,
    IssuerUrl,
    AuthorizationUrl,
    AuthorizationUrlParams,
    TokenUrl,
    TokenUrlParams,
    IntrospectionUrl,
    IntrospectionUrlParams,
    RevocationUrl,
    RevocationUrlParams,
    UserInfoUrl,
    JsonWebKeySetUrl,
    JsonWebKeySet,
    PkceCodeChallenge,
}

#[derive(DeriveIden)]
enum OidcProviderConnection {
    Table,

    Identifier,
    TokenType,
    AccessToken,
    RefreshToken,
    IdToken,
    ExpiredAt,
    Scopes,

    ProviderId,
    UserId,

    FkOidcProviderConnectionProvider,
    FkOidcProviderConnectionUser,

    UniqueOidcProviderIdentifier,
}

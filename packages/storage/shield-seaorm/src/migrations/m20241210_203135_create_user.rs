use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::base::{Base, BaseTable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "entity")]
        {
            manager
                .create_table(
                    BaseTable::create(Entity::Table)
                        .col(ColumnDef::new(Entity::Name).text().not_null())
                        .to_owned(),
                )
                .await?;

            manager
                .create_table(
                    BaseTable::create(User::Table, manager)
                        .col(ColumnDef::new(User::EntityId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name(User::FkUserEntity.to_string())
                                .from(User::Table, User::EntityId)
                                .to(Entity::Table, Base::Id)
                                .on_update(ForeignKeyAction::Cascade)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            manager
                .create_table(
                    BaseTable::create(EmailAddress::Table, manager)
                        .col(
                            ColumnDef::new(EmailAddress::Email)
                                .string_len(254)
                                .not_null()
                                .unique_key(),
                        )
                        .col(
                            ColumnDef::new(EmailAddress::IsPrimary)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .col(
                            ColumnDef::new(EmailAddress::IsVerified)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .col(ColumnDef::new(EmailAddress::VerificationToken).string_len(32))
                        .col(ColumnDef::new(EmailAddress::VerificationTokenExpiresAt).timestamp())
                        .col(ColumnDef::new(EmailAddress::VerifiedAt).timestamp())
                        .col(ColumnDef::new(EmailAddress::EntityId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name(EmailAddress::FkEmailAddressEntity.to_string())
                                .from(EmailAddress::Table, EmailAddress::EntityId)
                                .to(Entity::Table, Base::Id)
                                .on_update(ForeignKeyAction::Cascade)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;
        }

        #[cfg(not(feature = "entity"))]
        {
            manager
                .create_table(
                    BaseTable::create(User::Table, manager)
                        .col(ColumnDef::new(User::Name).text().not_null())
                        .to_owned(),
                )
                .await?;

            manager
                .create_table(
                    BaseTable::create(EmailAddress::Table, manager)
                        .col(
                            ColumnDef::new(EmailAddress::Email)
                                .string_len(254)
                                .not_null()
                                .unique_key(),
                        )
                        .col(
                            ColumnDef::new(EmailAddress::IsPrimary)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .col(
                            ColumnDef::new(EmailAddress::IsVerified)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .col(ColumnDef::new(EmailAddress::VerificationToken).string_len(32))
                        .col(ColumnDef::new(EmailAddress::VerificationTokenExpiresAt).timestamp())
                        .col(ColumnDef::new(EmailAddress::VerifiedAt).timestamp())
                        .col(ColumnDef::new(EmailAddress::UserId).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name(EmailAddress::FkEmailAddressUser.to_string())
                                .from(EmailAddress::Table, EmailAddress::UserId)
                                .to(User::Table, Base::Id)
                                .on_update(ForeignKeyAction::Cascade)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailAddress::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        #[cfg(feature = "entity")]
        {
            manager
                .drop_table(Table::drop().table(Entity::Table).to_owned())
                .await?;
        }

        Ok(())
    }
}

#[cfg(feature = "entity")]
#[derive(DeriveIden)]
enum Entity {
    Table,

    Name,
}

#[derive(DeriveIden)]
enum User {
    Table,

    #[cfg(not(feature = "entity"))]
    Name,

    #[cfg(feature = "entity")]
    EntityId,

    #[cfg(feature = "entity")]
    FkUserEntity,
}

#[derive(DeriveIden)]
enum EmailAddress {
    Table,

    Email,
    IsPrimary,
    IsVerified,
    VerificationToken,
    VerificationTokenExpiresAt,
    VerifiedAt,

    #[cfg(feature = "entity")]
    EntityId,
    #[cfg(not(feature = "entity"))]
    UserId,

    #[cfg(feature = "entity")]
    FkEmailAddressEntity,
    #[cfg(not(feature = "entity"))]
    FkEmailAddressUser,
}

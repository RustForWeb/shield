//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[cfg(not(feature = "entity"))]
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[cfg(feature = "entity")]
    pub entity_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[cfg(feature = "entity")]
    #[sea_orm(
        belongs_to = "super::entity::Entity",
        from = "Column::EntityId",
        to = "super::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Entity,
    #[cfg(not(feature = "entity"))]
    #[sea_orm(has_many = "super::email_address::Entity")]
    EmailAddress,
    #[cfg(feature = "provider-oauth")]
    #[sea_orm(has_many = "super::oauth_provider_connection::Entity")]
    OauthProviderConnection,
    #[cfg(feature = "provider-oidc")]
    #[sea_orm(has_many = "super::oidc_provider_connection::Entity")]
    OidcProviderConnection,
}

#[cfg(feature = "entity")]
impl Related<super::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entity.def()
    }
}

#[cfg(not(feature = "entity"))]
impl Related<super::email_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailAddress.def()
    }
}

#[cfg(feature = "provider-oauth")]
impl Related<super::oauth_provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OauthProviderConnection.def()
    }
}

#[cfg(feature = "provider-oidc")]
impl Related<super::oidc_provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OidcProviderConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
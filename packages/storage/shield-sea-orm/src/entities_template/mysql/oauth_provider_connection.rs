//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "oauth_provider_connection")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(Some(16)))"
    )]
    pub id: Vec<u8>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub identifier: String,
    #[sea_orm(column_type = "Text")]
    pub token_type: String,
    #[sea_orm(column_type = "Text")]
    pub access_token: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    pub expired_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scopes: Option<String>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub provider_id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub user_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::oauth_provider::Entity",
        from = "Column::ProviderId",
        to = "super::oauth_provider::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OauthProvider,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::oauth_provider::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OauthProvider.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
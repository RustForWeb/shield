//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use super::sea_orm_active_enums::PkceCodeChallenge;
use super::sea_orm_active_enums::Type;
use super::sea_orm_active_enums::Visibility;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "oauth_provider")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(Some(16)))"
    )]
    pub id: Vec<u8>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub name: String,
    pub slug: Option<String>,
    pub r#type: Type,
    pub visibility: Visibility,
    #[sea_orm(column_type = "Text")]
    pub client_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub client_secret: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scopes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub redirect_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub authorization_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub authorization_url_params: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_url_params: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub introspection_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub introspection_url_params: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub revocation_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub revocation_url_params: Option<String>,
    pub pkce_code_challenge: PkceCodeChallenge,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::oauth_provider_connection::Entity")]
    OauthProviderConnection,
}

impl Related<super::oauth_provider_connection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OauthProviderConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

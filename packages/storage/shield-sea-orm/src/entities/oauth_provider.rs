//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "oauth_provider_pkce_code_challenge"
)]
pub enum OauthProviderPkceCodeChallenge {
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "plain")]
    Plain,
    #[sea_orm(string_value = "s256")]
    S256,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "oauth_provider_type"
)]
pub enum OauthProviderType {
    #[sea_orm(string_value = "custom")]
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "oauth_provider_visibility"
)]
pub enum OauthProviderVisibility {
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "unlisted")]
    Unlisted,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(as = OauthProvider))]
#[sea_orm(table_name = "oauth_provider")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub name: String,
    pub slug: Option<String>,
    pub r#type: OauthProviderType,
    pub visibility: OauthProviderVisibility,
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
    pub pkce_code_challenge: OauthProviderPkceCodeChallenge,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon_url: Option<String>,
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

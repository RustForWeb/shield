//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(as = OidcProviderConnection))]
#[sea_orm(table_name = "oidc_provider_connection")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub identifier: String,
    #[sea_orm(column_type = "Text")]
    pub token_type: String,
    #[sea_orm(column_type = "Text")]
    pub access_token: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_token: Option<String>,
    pub expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scopes: Option<String>,
    pub provider_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::oidc_provider::Entity",
        from = "Column::ProviderId",
        to = "super::oidc_provider::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OidcProvider,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::oidc_provider::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OidcProvider.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

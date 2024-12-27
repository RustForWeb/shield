use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use shield::StorageError;
use shield_oidc::{
    CreateOidcConnection, OidcConnection, OidcProviderPkceCodeChallenge, OidcProviderVisibility,
    OidcStorage, OidcSubprovider, UpdateOidcConnection,
};

use crate::{
    entities::{oidc_provider, oidc_provider_connection, user},
    storage::SeaOrmStorage,
};

#[async_trait]
impl OidcStorage<user::Model> for SeaOrmStorage {
    async fn oidc_subproviders(&self) -> Result<Vec<OidcSubprovider>, StorageError> {
        oidc_provider::Entity::find()
            .all(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .and_then(|subproviders| {
                subproviders
                    .into_iter()
                    .map(OidcSubprovider::try_from)
                    .collect()
            })
    }

    async fn oidc_subprovider_by_id(
        &self,
        subprovider_id: &str,
    ) -> Result<Option<OidcSubprovider>, StorageError> {
        oidc_provider::Entity::find_by_id(Self::parse_uuid(subprovider_id)?)
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .and_then(|subprovider| match subprovider {
                Some(subprovider) => OidcSubprovider::try_from(subprovider).map(Option::Some),
                None => Ok(None),
            })
    }

    async fn oidc_connection_by_identifier(
        &self,
        subprovider_id: &str,
        identifier: &str,
    ) -> Result<Option<OidcConnection>, StorageError> {
        oidc_provider_connection::Entity::find()
            .filter(
                oidc_provider_connection::Column::ProviderId.eq(Self::parse_uuid(subprovider_id)?),
            )
            .filter(oidc_provider_connection::Column::Identifier.eq(identifier))
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|connection| connection.map(OidcConnection::from))
    }

    async fn create_oidc_connection(
        &self,
        connection: CreateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        let active_model = oidc_provider_connection::ActiveModel {
            identifier: ActiveValue::Set(connection.identifier),
            token_type: ActiveValue::Set(connection.token_type),
            access_token: ActiveValue::Set(connection.access_token),
            refresh_token: ActiveValue::Set(connection.refresh_token),
            id_token: ActiveValue::Set(connection.id_token),
            expired_at: ActiveValue::Set(connection.expired_at),
            scopes: ActiveValue::Set(connection.scopes.map(|scopes| scopes.join(","))),
            provider_id: ActiveValue::Set(Self::parse_uuid(&connection.subprovider_id)?),
            user_id: ActiveValue::Set(Self::parse_uuid(&connection.user_id)?),
            ..Default::default()
        };

        active_model
            .insert(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(OidcConnection::from)
    }

    async fn update_oidc_connection(
        &self,
        connection: UpdateOidcConnection,
    ) -> Result<OidcConnection, StorageError> {
        let mut active_model: oidc_provider_connection::ActiveModel =
            oidc_provider_connection::Entity::find()
                .filter(oidc_provider_connection::Column::Id.eq(Self::parse_uuid(&connection.id)?))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))?
                .ok_or_else(|| StorageError::NotFound("OIDC Connection".to_owned(), connection.id))?
                .into();

        if let Some(token_type) = connection.token_type {
            active_model.token_type = ActiveValue::Set(token_type);
        }
        if let Some(access_token) = connection.access_token {
            active_model.access_token = ActiveValue::Set(access_token);
        }
        if let Some(refresh_token) = connection.refresh_token {
            active_model.refresh_token = ActiveValue::Set(refresh_token);
        }
        if let Some(id_token) = connection.id_token {
            active_model.id_token = ActiveValue::Set(id_token);
        }
        if let Some(expired_at) = connection.expired_at {
            active_model.expired_at = ActiveValue::Set(expired_at);
        }
        if let Some(scopes) = connection.scopes {
            active_model.scopes = ActiveValue::Set(scopes.map(|scopes| scopes.join(",")));
        }

        active_model
            .update(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(OidcConnection::from)
    }

    async fn delete_oidc_connection(&self, connection_id: &str) -> Result<(), StorageError> {
        oidc_provider_connection::Entity::delete_by_id(Self::parse_uuid(connection_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}

impl From<oidc_provider::OidcProviderVisibility> for OidcProviderVisibility {
    fn from(value: oidc_provider::OidcProviderVisibility) -> Self {
        match value {
            oidc_provider::OidcProviderVisibility::Public => OidcProviderVisibility::Public,
            oidc_provider::OidcProviderVisibility::Unlisted => OidcProviderVisibility::Unlisted,
        }
    }
}

impl From<oidc_provider::OidcProviderPkceCodeChallenge> for OidcProviderPkceCodeChallenge {
    fn from(value: oidc_provider::OidcProviderPkceCodeChallenge) -> Self {
        match value {
            oidc_provider::OidcProviderPkceCodeChallenge::None => {
                OidcProviderPkceCodeChallenge::None
            }
            oidc_provider::OidcProviderPkceCodeChallenge::Plain => {
                OidcProviderPkceCodeChallenge::Plain
            }
            oidc_provider::OidcProviderPkceCodeChallenge::S256 => {
                OidcProviderPkceCodeChallenge::S256
            }
        }
    }
}

impl TryFrom<oidc_provider::Model> for OidcSubprovider {
    type Error = StorageError;

    fn try_from(value: oidc_provider::Model) -> Result<Self, Self::Error> {
        Ok(OidcSubprovider {
            id: value.id.to_string(),
            name: value.name,
            slug: value.slug,
            visibility: value.visibility.into(),
            client_id: value.client_id,
            client_secret: value.client_secret,
            scopes: value
                .scopes
                .map(|scopes| scopes.split(',').map(|s| s.to_string()).collect()),
            redirect_url: value.redirect_url,
            discovery_url: value.discovery_url,
            issuer_url: value.issuer_url,
            authorization_url: value.authorization_url,
            authorization_url_params: value.authorization_url_params,
            token_url: value.token_url,
            token_url_params: value.token_url_params,
            introspection_url: value.introspection_url,
            introspection_url_params: value.introspection_url_params,
            revocation_url: value.revocation_url,
            revocation_url_params: value.revocation_url_params,
            user_info_url: value.user_info_url,
            json_web_key_set_url: value.json_web_key_set_url,
            json_web_key_set: match value.json_web_key_set {
                Some(json_web_key_set) => serde_json::from_value(json_web_key_set)
                    .map_err(|err| StorageError::Validation(err.to_string()))?,
                None => None,
            },
            pkce_code_challenge: value.pkce_code_challenge.into(),
        })
    }
}

impl From<oidc_provider_connection::Model> for OidcConnection {
    fn from(value: oidc_provider_connection::Model) -> Self {
        OidcConnection {
            id: value.id.to_string(),
            identifier: value.identifier,
            token_type: value.token_type,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            id_token: value.id_token,
            expired_at: value.expired_at,
            scopes: value
                .scopes
                .map(|scopes| scopes.split(',').map(|s| s.to_string()).collect()),
            subprovider_id: value.provider_id.to_string(),
            user_id: value.user_id.to_string(),
        }
    }
}

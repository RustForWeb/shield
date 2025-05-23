use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use shield::StorageError;
use shield_oauth::{
    CreateOauthConnection, OauthConnection, OauthProvider, OauthProviderPkceCodeChallenge,
    OauthProviderVisibility, OauthStorage, UpdateOauthConnection,
};

use crate::{
    entities::{oauth_provider, oauth_provider_connection},
    storage::SeaOrmStorage,
    user::User,
};

#[async_trait]
impl OauthStorage<User> for SeaOrmStorage {
    async fn oauth_providers(&self) -> Result<Vec<OauthProvider>, StorageError> {
        oauth_provider::Entity::find()
            .all(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .and_then(|providers| providers.into_iter().map(OauthProvider::try_from).collect())
    }

    async fn oauth_provider_by_id_or_slug(
        &self,
        provider_id: &str,
    ) -> Result<Option<OauthProvider>, StorageError> {
        let condition = match Self::parse_uuid(provider_id) {
            Ok(provider_id) => oauth_provider::Column::Id.eq(provider_id),
            Err(_) => oauth_provider::Column::Slug.eq(provider_id.to_lowercase()),
        };

        oauth_provider::Entity::find()
            .filter(condition)
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .and_then(|provider| match provider {
                Some(provider) => OauthProvider::try_from(provider).map(Option::Some),
                None => Ok(None),
            })
    }

    async fn oauth_connection_by_id(
        &self,
        connection_id: &str,
    ) -> Result<Option<OauthConnection>, StorageError> {
        oauth_provider_connection::Entity::find_by_id(Self::parse_uuid(connection_id)?)
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|connection| connection.map(OauthConnection::from))
    }

    async fn oauth_connection_by_identifier(
        &self,
        provider_id: &str,
        identifier: &str,
    ) -> Result<Option<OauthConnection>, StorageError> {
        oauth_provider_connection::Entity::find()
            .filter(
                oauth_provider_connection::Column::ProviderId.eq(Self::parse_uuid(provider_id)?),
            )
            .filter(oauth_provider_connection::Column::Identifier.eq(identifier))
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|connection| connection.map(OauthConnection::from))
    }

    async fn create_oauth_connection(
        &self,
        connection: CreateOauthConnection,
    ) -> Result<OauthConnection, StorageError> {
        let active_model = oauth_provider_connection::ActiveModel {
            identifier: ActiveValue::Set(connection.identifier),
            token_type: ActiveValue::Set(connection.token_type),
            access_token: ActiveValue::Set(connection.access_token),
            refresh_token: ActiveValue::Set(connection.refresh_token),
            expired_at: ActiveValue::Set(connection.expired_at),
            scopes: ActiveValue::Set(connection.scopes.map(|scopes| scopes.join(","))),
            provider_id: ActiveValue::Set(Self::parse_uuid(&connection.provider_id)?),
            user_id: ActiveValue::Set(Self::parse_uuid(&connection.user_id)?),
            ..Default::default()
        };

        active_model
            .insert(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(OauthConnection::from)
    }

    async fn update_oauth_connection(
        &self,
        connection: UpdateOauthConnection,
    ) -> Result<OauthConnection, StorageError> {
        let mut active_model: oauth_provider_connection::ActiveModel =
            oauth_provider_connection::Entity::find()
                .filter(oauth_provider_connection::Column::Id.eq(Self::parse_uuid(&connection.id)?))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))?
                .ok_or_else(|| StorageError::NotFound("OauthConnection".to_owned(), connection.id))?
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
            .map(OauthConnection::from)
    }

    async fn delete_oauth_connection(&self, connection_id: &str) -> Result<(), StorageError> {
        oauth_provider_connection::Entity::delete_by_id(Self::parse_uuid(connection_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}

impl From<oauth_provider::OauthProviderVisibility> for OauthProviderVisibility {
    fn from(value: oauth_provider::OauthProviderVisibility) -> Self {
        match value {
            oauth_provider::OauthProviderVisibility::Public => OauthProviderVisibility::Public,
            oauth_provider::OauthProviderVisibility::Unlisted => OauthProviderVisibility::Unlisted,
        }
    }
}

impl From<oauth_provider::OauthProviderPkceCodeChallenge> for OauthProviderPkceCodeChallenge {
    fn from(value: oauth_provider::OauthProviderPkceCodeChallenge) -> Self {
        match value {
            oauth_provider::OauthProviderPkceCodeChallenge::None => {
                OauthProviderPkceCodeChallenge::None
            }
            oauth_provider::OauthProviderPkceCodeChallenge::Plain => {
                OauthProviderPkceCodeChallenge::Plain
            }
            oauth_provider::OauthProviderPkceCodeChallenge::S256 => {
                OauthProviderPkceCodeChallenge::S256
            }
        }
    }
}

impl TryFrom<oauth_provider::Model> for OauthProvider {
    type Error = StorageError;

    fn try_from(value: oauth_provider::Model) -> Result<Self, Self::Error> {
        Ok(OauthProvider {
            id: value.id.to_string(),
            name: value.name,
            slug: value.slug,
            icon_url: value.icon_url,
            visibility: value.visibility.into(),
            client_id: value.client_id,
            client_secret: value.client_secret,
            scopes: value
                .scopes
                .map(|scopes| scopes.split(',').map(|s| s.to_string()).collect()),
            redirect_url: value.redirect_url,
            authorization_url: value.authorization_url,
            authorization_url_params: value.authorization_url_params,
            token_url: value.token_url,
            token_url_params: value.token_url_params,
            introspection_url: value.introspection_url,
            introspection_url_params: value.introspection_url_params,
            revocation_url: value.revocation_url,
            revocation_url_params: value.revocation_url_params,
            pkce_code_challenge: value.pkce_code_challenge.into(),
        })
    }
}

impl From<oauth_provider_connection::Model> for OauthConnection {
    fn from(value: oauth_provider_connection::Model) -> Self {
        OauthConnection {
            id: value.id.to_string(),
            identifier: value.identifier,
            token_type: value.token_type,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expired_at: value.expired_at,
            scopes: value
                .scopes
                .map(|scopes| scopes.split(',').map(|s| s.to_string()).collect()),
            provider_id: value.provider_id.to_string(),
            user_id: value.user_id.to_string(),
        }
    }
}

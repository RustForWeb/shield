use async_trait::async_trait;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, TransactionError, TransactionTrait,
};
use shield::{CreateEmailAddress, CreateUser, Storage, StorageError, UpdateUser};

#[cfg(feature = "entity")]
use crate::entities::entity;
use crate::entities::{email_address, prelude::User, user};

pub const SEA_ORM_STORAGE_ID: &str = "sea-orm";

impl shield::User for user::Model {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct SeaOrmStorage {
    pub(crate) database: DatabaseConnection,
}

impl SeaOrmStorage {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub(crate) fn parse_uuid(uuid: &str) -> Result<Uuid, StorageError> {
        Uuid::parse_str(uuid).map_err(|err| StorageError::Validation(err.to_string()))
    }
}

#[async_trait]
impl Storage<user::Model> for SeaOrmStorage {
    fn id(&self) -> String {
        SEA_ORM_STORAGE_ID.to_owned()
    }

    async fn user_by_id(&self, user_id: &str) -> Result<Option<user::Model>, StorageError> {
        User::find_by_id(Self::parse_uuid(user_id)?)
            .one(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
    }

    async fn user_by_email(&self, email: &str) -> Result<Option<user::Model>, StorageError> {
        #[cfg(feature = "entity")]
        {
            use sea_orm::{JoinType, QuerySelect, RelationTrait};

            User::find()
                .join(JoinType::LeftJoin, user::Relation::Entity.def())
                .join(JoinType::LeftJoin, entity::Relation::EmailAddress.def())
                .filter(email_address::Column::Email.eq(email))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
        }
        #[cfg(not(feature = "entity"))]
        {
            User::find()
                .left_join(email_address::Entity)
                .filter(email_address::Column::Email.eq(email))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
        }
    }

    async fn create_user(
        &self,
        user: CreateUser,
        email_address: CreateEmailAddress,
    ) -> Result<user::Model, StorageError> {
        self.database
            .transaction::<_, user::Model, StorageError>(|database_transaction| {
                Box::pin(async move {
                    #[cfg(feature = "entity")]
                    {
                        let active_model = entity::ActiveModel {
                            name: ActiveValue::Set(user.name.unwrap_or_default()),
                            ..Default::default()
                        };

                        let entity = active_model
                            .insert(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        let active_model = user::ActiveModel {
                            entity_id: ActiveValue::Set(entity.id),
                            ..Default::default()
                        };

                        let user = active_model
                            .insert(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        let active_model = email_address::ActiveModel {
                            email: ActiveValue::Set(email_address.email),
                            is_primary: ActiveValue::Set(email_address.is_primary),
                            is_verified: ActiveValue::Set(email_address.is_verified),
                            verification_token: ActiveValue::Set(email_address.verification_token),
                            verification_token_expired_at: ActiveValue::Set(
                                email_address.verification_token_expired_at,
                            ),
                            verified_at: ActiveValue::Set(email_address.verified_at),
                            entity_id: ActiveValue::Set(entity.id),
                            ..Default::default()
                        };

                        active_model
                            .insert(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        Ok(user)
                    }

                    #[cfg(not(feature = "entity"))]
                    {
                        let active_model = user::ActiveModel {
                            name: ActiveValue::Set(user.name.unwrap_or_default()),
                            ..Default::default()
                        };

                        let user = active_model
                            .insert(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        let active_model = email_address::ActiveModel {
                            email: ActiveValue::Set(email_address.email),
                            is_primary: ActiveValue::Set(email_address.is_primary),
                            is_verified: ActiveValue::Set(email_address.is_verified),
                            verification_token: ActiveValue::Set(email_address.verification_token),
                            verification_token_expired_at: ActiveValue::Set(
                                email_address.verification_token_expired_at,
                            ),
                            verified_at: ActiveValue::Set(email_address.verified_at),
                            user_id: ActiveValue::Set(user.id),
                            ..Default::default()
                        };

                        active_model
                            .insert(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        Ok(user)
                    }
                })
            })
            .await
            .map_err(|err| match err {
                TransactionError::Connection(err) => StorageError::Engine(err.to_string()),
                TransactionError::Transaction(err) => err,
            })
    }

    async fn update_user(&self, user: UpdateUser) -> Result<user::Model, StorageError> {
        self.database
            .transaction::<_, user::Model, StorageError>(|database_transaction| {
                Box::pin(async move {
                    let user_entity = user::Entity::find()
                        .filter(user::Column::Id.eq(Self::parse_uuid(&user.id)?))
                        .one(database_transaction)
                        .await
                        .map_err(|err| StorageError::Engine(err.to_string()))?
                        .ok_or_else(|| {
                            StorageError::NotFound("User".to_owned(), user.id.clone())
                        })?;

                    #[cfg(feature = "entity")]
                    {
                        use sea_orm::ModelTrait;

                        let mut entity_active_model: entity::ActiveModel = user_entity
                            .find_related(entity::Entity)
                            .one(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?
                            .ok_or_else(|| {
                                StorageError::NotFound(
                                    "Entity".to_owned(),
                                    user_entity.entity_id.to_string(),
                                )
                            })?
                            .into();

                        if let Some(Some(name)) = user.name {
                            entity_active_model.name = ActiveValue::Set(name);
                        }

                        entity_active_model
                            .update(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;
                    }

                    #[allow(unused_mut)]
                    let mut user_active_model: user::ActiveModel = user_entity.into();

                    #[cfg(not(feature = "entity"))]
                    if let Some(Some(name)) = user.name {
                        user_active_model.name = ActiveValue::Set(name);
                    }

                    user_active_model
                        .update(database_transaction)
                        .await
                        .map_err(|err| StorageError::Engine(err.to_string()))
                })
            })
            .await
            .map_err(|err| match err {
                TransactionError::Connection(err) => StorageError::Engine(err.to_string()),
                TransactionError::Transaction(err) => err,
            })
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), StorageError> {
        user::Entity::delete_by_id(Self::parse_uuid(user_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}

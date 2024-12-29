use async_trait::async_trait;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, TransactionError, TransactionTrait,
};
use shield::{CreateEmailAddress, CreateUser, Storage, StorageError, UpdateUser};

#[cfg(feature = "entity")]
use crate::entities::entity;
use crate::{
    entities::{email_address, user},
    user::User,
};

pub const SEA_ORM_STORAGE_ID: &str = "sea-orm";

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
impl Storage<User> for SeaOrmStorage {
    fn id(&self) -> String {
        SEA_ORM_STORAGE_ID.to_owned()
    }

    async fn user_by_id(&self, user_id: &str) -> Result<Option<User>, StorageError> {
        #[cfg(feature = "entity")]
        {
            let user_and_entity = user::Entity::find_by_id(Self::parse_uuid(user_id)?)
                .find_also_related(entity::Entity)
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))?;

            match user_and_entity {
                Some((user, Some(entity))) => {
                    Ok(Some(User::new(self.database.clone(), user, entity)))
                }
                Some((user, None)) => Err(StorageError::NotFound(
                    "Entity".to_owned(),
                    user.entity_id.to_string(),
                )),
                None => Ok(None),
            }
        }

        #[cfg(not(feature = "entity"))]
        {
            user::Entity::find_by_id(Self::parse_uuid(user_id)?)
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
                .map(|user| user.map(|user| User::new(self.database.clone(), user)))
        }
    }

    async fn user_by_email(&self, email: &str) -> Result<Option<User>, StorageError> {
        #[cfg(feature = "entity")]
        {
            use sea_orm::{JoinType, QuerySelect, RelationTrait};

            let user_and_entity = user::Entity::find()
                .find_also_related(entity::Entity)
                // .join(JoinType::LeftJoin, user::Relation::Entity.def())
                .join(JoinType::LeftJoin, entity::Relation::EmailAddress.def())
                .filter(email_address::Column::Email.eq(email))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))?;

            match user_and_entity {
                Some((user, Some(entity))) => {
                    Ok(Some(User::new(self.database.clone(), user, entity)))
                }
                Some((user, None)) => Err(StorageError::NotFound(
                    "Entity".to_owned(),
                    user.entity_id.to_string(),
                )),
                None => Ok(None),
            }
        }

        #[cfg(not(feature = "entity"))]
        {
            user::Entity::find()
                .left_join(email_address::Entity)
                .filter(email_address::Column::Email.eq(email))
                .one(&self.database)
                .await
                .map_err(|err| StorageError::Engine(err.to_string()))
                .map(|user| user.map(|user| User::new(self.database.clone(), user)))
        }
    }

    async fn create_user(
        &self,
        user: CreateUser,
        email_address: CreateEmailAddress,
    ) -> Result<User, StorageError> {
        #[cfg(feature = "entity")]
        type UserAndEntity = (user::Model, entity::Model);

        #[cfg(not(feature = "entity"))]
        type UserAndEntity = user::Model;

        let user_and_entity = self
            .database
            .transaction::<_, UserAndEntity, StorageError>(|database_transaction| {
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

                        Ok((user, entity))
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
            })?;

        #[cfg(feature = "entity")]
        {
            let (user, entity) = user_and_entity;
            Ok(User::new(self.database.clone(), user, entity))
        }

        #[cfg(not(feature = "entity"))]
        {
            let user = user_and_entity;
            Ok(User::new(self.database.clone(), user))
        }
    }

    async fn update_user(&self, user: UpdateUser) -> Result<User, StorageError> {
        #[cfg(feature = "entity")]
        type UserAndEntity = (user::Model, entity::Model);

        #[cfg(not(feature = "entity"))]
        type UserAndEntity = user::Model;

        let user_and_entity = self
            .database
            .transaction::<_, UserAndEntity, StorageError>(|database_transaction| {
                Box::pin(async move {
                    #[cfg(feature = "entity")]
                    {
                        use sea_orm::ModelTrait;

                        let user_entity = user::Entity::find()
                            .filter(user::Column::Id.eq(Self::parse_uuid(&user.id)?))
                            .one(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?
                            .ok_or_else(|| {
                                StorageError::NotFound("User".to_owned(), user.id.clone())
                            })?;

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

                        let entity = entity_active_model
                            .update(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?;

                        Ok((user_entity, entity))
                    }

                    #[cfg(not(feature = "entity"))]
                    {
                        let user_entity = user::Entity::find()
                            .filter(user::Column::Id.eq(Self::parse_uuid(&user.id)?))
                            .one(database_transaction)
                            .await
                            .map_err(|err| StorageError::Engine(err.to_string()))?
                            .ok_or_else(|| {
                                StorageError::NotFound("User".to_owned(), user.id.clone())
                            })?;

                        let mut user_active_model: user::ActiveModel = user_entity.into();

                        #[cfg(not(feature = "entity"))]
                        if let Some(Some(name)) = user.name {
                            user_active_model.name = ActiveValue::Set(name);
                        }

                        let user = user_active_model
                            .update(database_transaction)
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
            })?;

        #[cfg(feature = "entity")]
        {
            let (user, entity) = user_and_entity;
            Ok(User::new(self.database.clone(), user, entity))
        }

        #[cfg(not(feature = "entity"))]
        {
            let user = user_and_entity;
            Ok(User::new(self.database.clone(), user))
        }
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), StorageError> {
        user::Entity::delete_by_id(Self::parse_uuid(user_id)?)
            .exec(&self.database)
            .await
            .map_err(|err| StorageError::Engine(err.to_string()))
            .map(|_| ())
    }
}

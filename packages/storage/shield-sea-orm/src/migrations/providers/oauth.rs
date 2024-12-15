mod m20241211_095111_create_provider_oauth;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderOauthMigrator;

#[async_trait]
impl MigratorTrait for ProviderOauthMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            self::m20241211_095111_create_provider_oauth::Migration,
        )]
    }
}
mod m20241211_184751_create_provider_oidc;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderOidcMigrator;

#[async_trait]
impl MigratorTrait for ProviderOidcMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            self::m20241211_184751_create_provider_oidc::Migration,
        )]
    }
}

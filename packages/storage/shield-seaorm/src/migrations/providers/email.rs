use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20241211_092737_create_provider_email;

pub struct ProviderEmailMigrator;

#[async_trait]
impl MigratorTrait for ProviderEmailMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241211_092737_create_provider_email::Migration)]
    }
}

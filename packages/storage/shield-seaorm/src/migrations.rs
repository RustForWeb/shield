use core::CoreMigrator;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod core;
mod providers;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        #[allow(unused_mut)]
        let mut migrations = CoreMigrator::migrations();

        #[cfg(feature = "provider-email")]
        {
            use providers::email::ProviderEmailMigrator;
            migrations.extend(ProviderEmailMigrator::migrations());
        }

        migrations
    }
}

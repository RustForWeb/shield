pub mod core;
pub mod providers;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

use self::{core::CoreMigrator, providers::ProvidersMigrator};

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        CoreMigrator::migrations()
            .into_iter()
            .chain(ProvidersMigrator::migrations())
            .collect()
    }
}

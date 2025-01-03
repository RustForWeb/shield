mod m20241210_203135_create_user;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct CoreMigrator;

#[async_trait]
impl MigratorTrait for CoreMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(self::m20241210_203135_create_user::Migration)]
    }
}

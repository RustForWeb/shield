use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20241210_203135_create_user;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241210_203135_create_user::Migration)]
    }
}

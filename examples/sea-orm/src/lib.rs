use sea_orm::{Database, DbErr};
use sea_orm_migration::migrator::MigratorTrait;
use shield_sea_orm::{SeaOrmStorage, migrations::Migrator};

pub async fn initialize() -> Result<SeaOrmStorage, DbErr> {
    let database = Database::connect("sqlite::memory:").await?;

    Migrator::up(&database, None).await?;

    Ok(SeaOrmStorage::new(database))
}

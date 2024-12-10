use sea_orm::{Database, DbErr};
use sea_orm_migration::migrator::MigratorTrait;
use shield_seaorm::migrations::Migrator;

pub async fn run() -> Result<(), DbErr> {
    let database = Database::connect("sqlite::memory:").await?;

    Migrator::up(&database, None).await?;

    database.close().await?;

    Ok(())
}

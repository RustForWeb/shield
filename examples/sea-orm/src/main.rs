use sea_orm_migration::prelude::*;
use shield_sea_orm::migrations::Migrator;

#[tokio::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

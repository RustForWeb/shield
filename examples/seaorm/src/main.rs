use sea_orm_migration::prelude::*;
use shield_seaorm::migrations::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

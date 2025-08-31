use sea_orm::Database;
use sea_orm_migration::migrator::MigratorTrait;
use shield_sea_orm::migrations::Migrator;

const BACKENDS: &[(&str, &str)] = &[
    ("mysql", "mysql://shield:shield@localhost:13306/shield"),
    (
        "postgresql",
        "postgres://shield:shield@localhost:15432/shield",
    ),
    ("sqlite", "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc"),
];

#[tokio::test]
async fn migrations() {
    for (backend, url) in BACKENDS {
        let database = Database::connect(url.to_owned())
            .await
            .unwrap_or_else(|err| panic!("Connect to backend `{backend}` failed: {err}"));

        // Up migrations
        Migrator::fresh(&database)
            .await
            .unwrap_or_else(|err| panic!("Up migrations for backend `{backend}` failed: {err}"));

        // Down migrations
        Migrator::refresh(&database)
            .await
            .unwrap_or_else(|err| panic!("Down migrations for backend `{backend}` failed: {err}"));

        // Cleanup
        Migrator::reset(&database)
            .await
            .unwrap_or_else(|err| panic!("Cleanup for backend `{backend}` failed: {err}"));
    }
}

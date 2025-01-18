mod m20241211_184751_create_provider_oidc;
mod m20250118_133731_add_icon_url;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderOidcMigrator;

#[async_trait]
impl MigratorTrait for ProviderOidcMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(self::m20241211_184751_create_provider_oidc::Migration),
            Box::new(self::m20250118_133731_add_icon_url::Migration),
        ]
    }
}

mod m20241211_095111_create_provider_oauth;
mod m20250118_133257_add_icon_url;
mod m20260613_131851_add_user_url_and_paths;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderOauthMigrator;

#[async_trait]
impl MigratorTrait for ProviderOauthMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(self::m20241211_095111_create_provider_oauth::Migration),
            Box::new(self::m20250118_133257_add_icon_url::Migration),
            Box::new(self::m20260613_131851_add_user_url_and_paths::Migration),
        ]
    }
}

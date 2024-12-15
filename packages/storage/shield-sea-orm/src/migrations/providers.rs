#[cfg(feature = "provider-email")]
pub mod email;
#[cfg(feature = "provider-oauth")]
pub mod oauth;
#[cfg(feature = "provider-oidc")]
pub mod oidc;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProvidersMigrator;

#[async_trait]
impl MigratorTrait for ProvidersMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        #[allow(unused_mut)]
        let mut migrations = vec![];

        #[cfg(feature = "provider-email")]
        {
            use self::email::ProviderEmailMigrator;
            migrations.extend(ProviderEmailMigrator::migrations());
        }
        #[cfg(feature = "provider-oauth")]
        {
            use self::oauth::ProviderOauthMigrator;
            migrations.extend(ProviderOauthMigrator::migrations());
        }
        #[cfg(feature = "provider-oidc")]
        {
            use self::oidc::ProviderOidcMigrator;
            migrations.extend(ProviderOidcMigrator::migrations());
        }

        migrations
    }
}

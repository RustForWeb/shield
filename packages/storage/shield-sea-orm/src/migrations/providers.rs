#[cfg(feature = "method-email")]
pub mod email;
#[cfg(feature = "method-oauth")]
pub mod oauth;
#[cfg(feature = "method-oidc")]
pub mod oidc;

use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProvidersMigrator;

#[async_trait]
impl MigratorTrait for ProvidersMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        #[allow(unused_mut)]
        let mut migrations = vec![];

        #[cfg(feature = "method-email")]
        {
            use self::email::ProviderEmailMigrator;
            migrations.extend(ProviderEmailMigrator::migrations());
        }
        #[cfg(feature = "method-oauth")]
        {
            use self::oauth::ProviderOauthMigrator;
            migrations.extend(ProviderOauthMigrator::migrations());
        }
        #[cfg(feature = "method-oidc")]
        {
            use self::oidc::ProviderOidcMigrator;
            migrations.extend(ProviderOidcMigrator::migrations());
        }

        migrations
    }
}

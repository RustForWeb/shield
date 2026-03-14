use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use shield::ShieldError;

#[async_trait]
pub trait Sender: Send + Sync {
    async fn send(
        &self,
        email: &str,
        token: &str,
        expires_at: DateTime<FixedOffset>,
    ) -> Result<(), ShieldError>;
}

#[cfg(feature = "sender-tracing")]
mod tracing {
    use async_trait::async_trait;
    use chrono::{DateTime, FixedOffset};
    use shield::ShieldError;
    use tracing::info;

    use super::Sender;

    pub struct TracingSender;

    #[async_trait]
    impl Sender for TracingSender {
        async fn send(
            &self,
            email: &str,
            token: &str,
            expires_at: DateTime<FixedOffset>,
        ) -> Result<(), ShieldError> {
            info!("Email authentication token for `{email}` expires at `{expires_at}`:\n`{token}`");

            Ok(())
        }
    }
}

#[cfg(feature = "sender-tracing")]
pub use tracing::*;

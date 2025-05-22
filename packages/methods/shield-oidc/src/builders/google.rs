use crate::provider::{
    OidcProvider, OidcProviderBuilder,
    oidc_provider_builder::{SetClientId, SetDiscoveryUrl, SetIconUrl, SetId, SetName},
};

pub struct Google {}

impl Google {
    pub fn builder(
        id: &str,
        client_id: &str,
    ) -> OidcProviderBuilder<SetDiscoveryUrl<SetClientId<SetIconUrl<SetName<SetId>>>>> {
        OidcProvider::builder()
            .id(id)
            .name("Google")
            .icon_url("https://authjs.dev/img/providers/google.svg")
            .client_id(client_id)
            .discovery_url("https://accounts.google.com")
    }
}

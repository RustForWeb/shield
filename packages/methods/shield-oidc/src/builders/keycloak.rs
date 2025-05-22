use crate::provider::{
    OidcProvider, OidcProviderBuilder,
    oidc_provider_builder::{SetClientId, SetDiscoveryUrl, SetIconUrl, SetId, SetName},
};

pub struct Keycloak {}

impl Keycloak {
    pub fn builder(
        id: &str,
        discovery_url: &str,
        client_id: &str,
    ) -> OidcProviderBuilder<SetDiscoveryUrl<SetClientId<SetIconUrl<SetName<SetId>>>>> {
        OidcProvider::builder()
            .id(id)
            .name("Keycloak")
            .icon_url("https://authjs.dev/img/providers/keycloak.svg")
            .client_id(client_id)
            .discovery_url(discovery_url)
    }
}

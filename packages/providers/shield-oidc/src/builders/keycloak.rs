use crate::subprovider::{
    oidc_subprovider_builder::{SetClientId, SetDiscoveryUrl, SetId, SetName},
    OidcSubprovider, OidcSubproviderBuilder,
};

pub struct Keycloak {}

impl Keycloak {
    pub fn builder(
        id: &str,
        discovery_url: &str,
        client_id: &str,
    ) -> OidcSubproviderBuilder<SetDiscoveryUrl<SetClientId<SetName<SetId>>>> {
        OidcSubprovider::builder()
            .id(id)
            .name("Keycloak")
            .client_id(client_id)
            .discovery_url(discovery_url)
    }
}

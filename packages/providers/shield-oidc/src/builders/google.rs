use crate::subprovider::{
    oidc_subprovider_builder::{SetClientId, SetDiscoveryUrl, SetId, SetName},
    OidcSubprovider, OidcSubproviderBuilder,
};

pub struct Google {}

impl Google {
    pub fn builder(
        id: &str,
        client_id: &str,
    ) -> OidcSubproviderBuilder<SetDiscoveryUrl<SetClientId<SetName<SetId>>>> {
        OidcSubprovider::builder()
            .id(id)
            .name("Google")
            .client_id(client_id)
            .discovery_url("https://accounts.google.com")
    }
}

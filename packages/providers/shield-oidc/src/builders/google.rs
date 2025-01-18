use crate::subprovider::{
    oidc_subprovider_builder::{SetClientId, SetDiscoveryUrl, SetIconUrl, SetId, SetName},
    OidcSubprovider, OidcSubproviderBuilder,
};

pub struct Google {}

impl Google {
    pub fn builder(
        id: &str,
        client_id: &str,
    ) -> OidcSubproviderBuilder<SetDiscoveryUrl<SetClientId<SetIconUrl<SetName<SetId>>>>> {
        OidcSubprovider::builder()
            .id(id)
            .name("Google")
            .icon_url("https://authjs.dev/img/providers/google.svg")
            .client_id(client_id)
            .discovery_url("https://accounts.google.com")
    }
}

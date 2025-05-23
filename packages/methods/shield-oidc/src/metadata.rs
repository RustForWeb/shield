use openidconnect::{
    AdditionalProviderMetadata, IntrospectionUrl, ProviderMetadata, RevocationUrl,
    core::{
        CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClientAuthMethod, CoreGrantType,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJweKeyManagementAlgorithm,
        CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonStandardProviderMetadata {
    #[serde(default)]
    pub introspection_endpoint: Option<IntrospectionUrl>,
    #[serde(default)]
    pub revocation_endpoint: Option<RevocationUrl>,
}

impl AdditionalProviderMetadata for NonStandardProviderMetadata {}

pub type OidcProviderMetadata = ProviderMetadata<
    NonStandardProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

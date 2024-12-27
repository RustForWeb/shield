use openidconnect::{
    core::CoreGenderClaim, EmptyAdditionalClaims, EndUserEmail, EndUserName, IdTokenClaims,
    LocalizedClaim, SubjectIdentifier, UserInfoClaims,
};

/// Unified interface for [`IdTokenClaims`] and [`UserInfoClaims`].
#[derive(Clone, Debug)]
pub enum Claims {
    IdToken(IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>),
    UserInfo(UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim>),
}

impl Claims {
    pub fn subject(&self) -> &SubjectIdentifier {
        match &self {
            Claims::IdToken(id_token_claims) => id_token_claims.subject(),
            Claims::UserInfo(user_info_claims) => user_info_claims.subject(),
        }
    }

    pub fn email(&self) -> Option<&EndUserEmail> {
        match &self {
            Claims::IdToken(id_token_claims) => id_token_claims.email(),
            Claims::UserInfo(user_info_claims) => user_info_claims.email(),
        }
    }

    pub fn name(&self) -> Option<&LocalizedClaim<EndUserName>> {
        match &self {
            Claims::IdToken(id_token_claims) => id_token_claims.name(),
            Claims::UserInfo(user_info_claims) => user_info_claims.name(),
        }
    }
}

impl From<IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>> for Claims {
    fn from(value: IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>) -> Self {
        Self::IdToken(value)
    }
}

impl From<UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim>> for Claims {
    fn from(value: UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim>) -> Self {
        Self::UserInfo(value)
    }
}

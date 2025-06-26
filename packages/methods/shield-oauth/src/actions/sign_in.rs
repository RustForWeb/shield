use async_trait::async_trait;
use oauth2::{CsrfToken, PkceCodeChallenge, Scope, url::form_urlencoded::parse};
use shield::{
    Action, ConfigurationError, Form, Request, Response, SIGN_IN_ACTION_ID, Session, SessionError,
    ShieldError, erased_action,
};

use crate::{
    method::OAUTH_METHOD_ID,
    provider::{OauthProvider, OauthProviderPkceCodeChallenge},
    session::OauthSession,
};

pub struct OauthSignInAction;

#[async_trait]
impl Action<OauthProvider> for OauthSignInAction {
    fn id(&self) -> String {
        SIGN_IN_ACTION_ID.to_owned()
    }

    fn render(&self, _provider: OauthProvider) -> Form {
        Form {
            inputs: vec![],
            attributes: None,
        }
    }

    async fn call(
        &self,
        provider: OauthProvider,
        session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        let client = provider.oauth_client().await?;

        let mut authorization_request = client
            .authorize_url(CsrfToken::new_random)
            .map_err(|err| ConfigurationError::Invalid(err.to_string()))?;

        let pkce_code_challenge = match provider.pkce_code_challenge {
            OauthProviderPkceCodeChallenge::None => None,
            OauthProviderPkceCodeChallenge::Plain => Some(PkceCodeChallenge::new_random_plain()),
            OauthProviderPkceCodeChallenge::S256 => Some(PkceCodeChallenge::new_random_sha256()),
        };

        if let Some((pkce_code_challenge, _)) = &pkce_code_challenge {
            authorization_request =
                authorization_request.set_pkce_challenge(pkce_code_challenge.clone());
        }

        if let Some(scopes) = provider.scopes {
            authorization_request =
                authorization_request.add_scopes(scopes.into_iter().map(Scope::new));
        }

        if let Some(authorization_url_params) = provider.authorization_url_params {
            let params = parse(authorization_url_params.trim_start_matches('?').as_bytes());

            for (name, value) in params {
                authorization_request =
                    authorization_request.add_extra_param(name.into_owned(), value.into_owned());
            }
        }

        let (auth_url, csrf_token) = authorization_request.url();

        {
            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = None;

            session_data.set_method(
                OAUTH_METHOD_ID,
                OauthSession {
                    csrf: Some(csrf_token.secret().clone()),
                    pkce_verifier: pkce_code_challenge
                        .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone()),
                    oauth_connection_id: None,
                },
            )?;
        }

        Ok(Response::Redirect(auth_url.to_string()))
    }
}

erased_action!(OauthSignInAction);

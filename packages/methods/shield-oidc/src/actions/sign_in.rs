use async_trait::async_trait;
use openidconnect::{
    CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow,
    url::form_urlencoded::parse,
};
use shield::{
    Action, Form, Input, InputType, InputTypeSubmit, Provider, Request, Response, Session,
    SessionError, ShieldError, SignInAction, erased_action,
};

use crate::{
    method::OIDC_METHOD_ID,
    provider::{OidcProvider, OidcProviderPkceCodeChallenge},
    session::OidcSession,
};

pub struct OidcSignInAction;

#[async_trait]
impl Action<OidcProvider> for OidcSignInAction {
    fn id(&self) -> String {
        SignInAction::id()
    }

    fn name(&self) -> String {
        SignInAction::name()
    }

    fn forms(&self, provider: OidcProvider) -> Vec<Form> {
        vec![Form {
            inputs: vec![Input {
                name: "submit".to_owned(),
                label: None,
                r#type: InputType::Submit(InputTypeSubmit::default()),
                value: Some(format!("Sign in with {}", provider.name())),
            }],
        }]
    }

    async fn call(
        &self,
        provider: OidcProvider,
        session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        let client = provider.oidc_client().await?;

        let mut authorization_request = client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );

        let pkce_code_challenge = match provider.pkce_code_challenge {
            OidcProviderPkceCodeChallenge::None => None,
            OidcProviderPkceCodeChallenge::Plain => Some(PkceCodeChallenge::new_random_plain()),
            OidcProviderPkceCodeChallenge::S256 => Some(PkceCodeChallenge::new_random_sha256()),
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

        let (auth_url, csrf_token, nonce) = authorization_request.url();

        {
            // TODO: Add a generic type for session data to actions, so the action caller can be read/write the session.

            let session_data = session.data();
            let mut session_data = session_data
                .lock()
                .map_err(|err| SessionError::Lock(err.to_string()))?;

            session_data.authentication = None;

            session_data.set_method(
                OIDC_METHOD_ID,
                OidcSession {
                    csrf: Some(csrf_token.secret().clone()),
                    nonce: Some(nonce.secret().clone()),
                    pkce_verifier: pkce_code_challenge
                        .map(|(_, pkce_code_verifier)| pkce_code_verifier.secret().clone()),
                    oidc_connection_id: None,
                },
            )?;
        }

        Ok(Response::Redirect(auth_url.to_string()))
    }
}

erased_action!(OidcSignInAction);

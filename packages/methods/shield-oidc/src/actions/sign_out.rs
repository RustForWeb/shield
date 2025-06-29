use async_trait::async_trait;
use shield::{Action, Form, Request, Response, Session, ShieldError, SignOutAction, erased_action};

use crate::provider::OidcProvider;

pub struct OidcSignOutAction;

#[async_trait]
impl Action<OidcProvider> for OidcSignOutAction {
    fn id(&self) -> String {
        SignOutAction::id()
    }

    fn name(&self) -> String {
        SignOutAction::name()
    }

    fn condition(&self, provider: &OidcProvider, session: Session) -> Result<bool, ShieldError> {
        SignOutAction::condition(provider, session)
    }

    fn form(&self, provider: OidcProvider) -> Form {
        SignOutAction::form(provider)
    }

    async fn call(
        &self,
        _provider: OidcProvider,
        _session: Session,
        _request: Request,
    ) -> Result<Response, ShieldError> {
        // TODO: See [`OidcProvider::oidc_client`].

        // let provider = match request.provider_id {
        //     Some(provider_id) => self.oidc_provider_by_id_or_slug(&provider_id).await?,
        //     None => return Err(ProviderError::ProviderMissing.into()),
        // };

        // let connection_id = {
        //     let session_data = session.data();
        //     let session_data = session_data
        //         .lock()
        //         .map_err(|err| SessionError::Lock(err.to_string()))?;

        //     session_data.oidc_connection_id.clone()
        // };

        // if let Some(connection_id) = connection_id {
        //     if let Some(connection) = self.storage.oidc_connection_by_id(&connection_id).await? {
        //         debug!("revoking access token {:?}", connection.access_token);

        //         let token = AccessToken::new(connection.access_token);

        //         let client = subprovider.oidc_client().await?;

        //         let revocation_request = match client.revoke_token(token.into()) {
        //             Ok(revocation_request) => Some(revocation_request),
        //             Err(openidconnect::ConfigurationError::MissingUrl("revocation")) => None,
        //             Err(err) => return Err(ConfigurationError::Invalid(err.to_string()).into()),
        //         };

        //         if let Some(revocation_request) = revocation_request {
        //             let mut revocation_request = revocation_request;

        //             if let Some(revocation_url_params) = subprovider.revocation_url_params {
        //                 let params =
        //                     parse(revocation_url_params.trim_start_matches('?').as_bytes());

        //                 for (name, value) in params {
        //                     revocation_request = revocation_request
        //                         .add_extra_param(name.into_owned(), value.into_owned());
        //                 }
        //             }

        //             revocation_request
        //                 .request_async(async_http_client)
        //                 .await
        //                 .map_err(|err| ShieldError::Request(err.to_string()))?;
        //         }
        //     }
        // }

        // TODO: Sign out.

        Ok(Response::Default)
    }
}

erased_action!(OidcSignOutAction);

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use shield::{ShieldError, User};

use crate::{error::RouteError, ExtractShield, ExtractUser};

pub async fn auth_required<U: User>(
    ExtractUser(user): ExtractUser<U>,
    request: Request,
    next: Next,
) -> Response {
    match user {
        Some(_) => next.run(request).await,
        None => RouteError::from(ShieldError::Unauthorized).into_response(),
    }
}

pub async fn auth_required_redirect<U: User>(
    ExtractShield(shield): ExtractShield<U>,
    ExtractUser(user): ExtractUser<U>,
    request: Request,
    next: Next,
) -> Response {
    match user {
        Some(_) => next.run(request).await,
        None => Redirect::to(&shield.options().sign_in_redirect).into_response(),
    }
}

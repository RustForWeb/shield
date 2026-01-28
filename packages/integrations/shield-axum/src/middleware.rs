use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shield::{ShieldError, User};

use crate::{ExtractUser, error::RouteError};

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

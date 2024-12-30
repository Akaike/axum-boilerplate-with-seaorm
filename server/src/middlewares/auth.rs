use axum::{extract::Request, middleware::Next, response::Response};
use reqwest::header;

use crate::{
    error::{AuthError, Error},
    utils::jwt,
};

pub async fn is_authenticated(mut req: Request, next: Next) -> Result<Response, Error> {
    let token = extract_token(&req)?;
    let claims = jwt::validate(token).await?;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

fn extract_token(req: &Request) -> Result<&str, Error> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
        .ok_or(Error::Auth(AuthError::MissingToken))
}

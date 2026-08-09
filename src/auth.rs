use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
    Extension,
    extract::State,
};

use crate::{
    error::ApiError,
    models::CurrentUser,
    AppState,
};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(ApiError::Unauthorized)?;

    let user = state.supabase.authenticate(token).await?;

    request.extensions_mut().insert(CurrentUser {
        id: user.id,
        email: user.email,
    });

    // Keep the access token available to handlers without exposing it
    // through JSON responses.
    request.extensions_mut().insert(AccessToken(token.to_owned()));

    Ok(next.run(request).await)
}

#[derive(Clone, Debug)]
pub struct AccessToken(pub String);

pub fn current_user(
    extensions: &http::Extensions,
) -> Result<&CurrentUser, ApiError> {
    extensions
        .get::<CurrentUser>()
        .ok_or(ApiError::Unauthorized)
}

pub fn access_token(
    extensions: &http::Extensions,
) -> Result<&AccessToken, ApiError> {
    extensions
        .get::<AccessToken>()
        .ok_or(ApiError::Unauthorized)
}

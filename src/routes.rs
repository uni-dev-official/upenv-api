use axum::{
    extract::{Extension, Path, State},
    middleware,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::{access_token, current_user, require_auth},
    error::ApiError,
    models::{Backup, CreateBackup, CreateDevice, CurrentUser, Device},
    AppState,
};

pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/api/me", get(me))
        .route("/api/devices", get(list_devices).post(create_device))
        .route("/api/devices/{id}", delete(delete_device))
        .route("/api/backups", get(list_backups).post(create_backup))
        .route("/api/backups/{id}", delete(delete_backup))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ));

    Router::new()
        .route("/health", get(health))
        .merge(protected)
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "upenv-api"
    }))
}

async fn me(
    Extension(user): Extension<CurrentUser>,
) -> Json<CurrentUser> {
    Json(user)
}

async fn list_devices(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
) -> Result<Json<Vec<Device>>, ApiError> {
    let path = format!(
        "devices?select=*&user_id=eq.{}&order=created_at.desc",
        user.id
    );

    let devices = state.supabase.get(&path, &token.0).await?;
    Ok(Json(devices))
}

async fn create_device(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
    Json(input): Json<CreateDevice>,
) -> Result<Json<Device>, ApiError> {
    if input.name.trim().is_empty() || input.platform.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "name and platform are required".into(),
        ));
    }

    let payload = json!({
        "user_id": user.id,
        "name": input.name,
        "platform": input.platform,
        "architecture": input.architecture,
        "app_version": input.app_version
    });

    let rows: Vec<Device> = state
        .supabase
        .post("devices", &token.0, &payload)
        .await?;

    rows.into_iter()
        .next()
        .map(Json)
        .ok_or(ApiError::Internal)
}

async fn delete_device(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let path = format!("devices?id=eq.{id}&user_id=eq.{}", user.id);

    let _: Vec<serde_json::Value> = state.supabase.delete(&path, &token.0).await?;

    Ok(Json(json!({ "deleted": true })))
}

async fn list_backups(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
) -> Result<Json<Vec<Backup>>, ApiError> {
    let path = format!(
        "backups?select=*&user_id=eq.{}&order=created_at.desc",
        user.id
    );

    let backups = state.supabase.get(&path, &token.0).await?;
    Ok(Json(backups))
}

async fn create_backup(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
    Json(input): Json<CreateBackup>,
) -> Result<Json<Backup>, ApiError> {
    if input.name.trim().is_empty() {
        return Err(ApiError::BadRequest("name is required".into()));
    }

    let payload = json!({
        "user_id": user.id,
        "device_id": input.device_id,
        "name": input.name,
        "manifest": input.manifest,
        "status": "ready"
    });

    let rows: Vec<Backup> = state
        .supabase
        .post("backups", &token.0, &payload)
        .await?;

    rows.into_iter()
        .next()
        .map(Json)
        .ok_or(ApiError::Internal)
}

async fn delete_backup(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
    Extension(token): Extension<crate::auth::AccessToken>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let path = format!("backups?id=eq.{id}&user_id=eq.{}", user.id);

    let _: Vec<serde_json::Value> = state.supabase.delete(&path, &token.0).await?;

    Ok(Json(json!({ "deleted": true })))
}

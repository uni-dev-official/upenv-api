use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    error::AppError,
    models::{Backup, ForkBackupRequest},
    AppState,
};

/// Endpoint: POST /api/backups/:id/fork
pub async fn fork_backup(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(backup_id): Path<Uuid>,
    Json(payload): Json<ForkBackupRequest>,
) -> Result<(StatusCode, Json<Backup>), AppError> {
    // 1. Fetch source backup (RLS allows fetch if is_public = true OR user_id = user.id)
    let source_backup = state
        .supabase
        .get_backup_by_id(backup_id, &user)
        .await?
        .ok_or_else(|| AppError::NotFound("Backup not found or is private".into()))?;

    // Check if user is trying to fork a private backup owned by someone else
    if !source_backup.is_public && source_backup.user_id != user.id {
        return Err(AppError::Forbidden("Cannot fork a private backup".into()));
    }

    // 2. Verify target device exists and belongs to the authenticated user
    let target_device = state
        .supabase
        .get_device_by_id(payload.target_device_id, &user)
        .await?
        .ok_or_else(|| AppError::BadRequest("Target device does not exist or does not belong to you".into()))?;

    if target_device.user_id != user.id {
        return Err(AppError::Forbidden("Target device does not belong to you".into()));
    }

    // 3. Prepare the new cloned backup data
    let new_name = payload.name.unwrap_or_else(|| {
        format!("{} (Fork)", source_backup.name)
    });

    // 4. Create the cloned backup for the target user
    let cloned_backup = state
        .supabase
        .create_backup(
            &user,
            payload.target_device_id,
            &new_name,
            source_backup.manifest,
            false, // Default cloned backups to private
        )
        .await?;

    // 5. Asynchronously/best-effort increment fork_count on original backup
    let supabase = state.supabase.clone();
    tokio::spawn(async move {
        let _ = supabase.increment_fork_count(backup_id).await;
    });

    Ok((StatusCode::CREATED, Json(cloned_backup)))
}
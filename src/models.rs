use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SupabaseUser {
    pub id: Uuid,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDevice {
    pub name: String,
    pub platform: String,
    pub architecture: Option<String>,
    pub app_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub platform: String,
    pub architecture: Option<String>,
    pub app_version: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBackup {
    pub device_id: Uuid,
    pub name: String,
    pub manifest: serde_json::Value,
    #[serde(default)]
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub name: String,
    pub manifest: serde_json::Value,
    pub status: String,

    pub is_public: bool,
    pub fork_count: Option<i32>,

    pub forked_from: Option<Uuid>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Request payload for copying an existing (public) backup onto one of the user's target devices
#[derive(Debug, Deserialize)]
pub struct ForkBackupRequest {
    pub target_device_id: Uuid,
    /// Optional custom name for the new backup (e.g., "Cloned from @alex's Mac Setup")
    pub name: Option<String>,
}

/// Filter options for fetching/browsing community workstation setups
#[derive(Debug, Deserialize)]
pub struct PublicBackupsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
}
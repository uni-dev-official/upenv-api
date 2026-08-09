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

#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
pub struct Backup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub name: String,
    pub manifest: serde_json::Value,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

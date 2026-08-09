use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::de::DeserializeOwned;

use crate::error::ApiError;

#[derive(Clone)]
pub struct SupabaseClient {
    http: Client,
    base_url: String,
    anon_key: String,
}

impl SupabaseClient {
    pub fn new(base_url: String, anon_key: String) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.trim_end_matches('/').to_owned(),
            anon_key,
        }
    }

    pub async fn authenticate(
        &self,
        access_token: &str,
    ) -> Result<crate::models::SupabaseUser, ApiError> {
        let url = format!("{}/auth/v1/user", self.base_url);

        let response = self.http
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .header("apikey", &self.anon_key)
            .send()
            .await
            .map_err(|_| ApiError::Unauthorized)?;

        if !response.status().is_success() {
            return Err(ApiError::Unauthorized);
        }

        response
            .json()
            .await
            .map_err(|_| ApiError::Unauthorized)
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
    ) -> Result<T, ApiError> {
        self.request(reqwest::Method::GET, path, access_token, None::<&()>)
            .await
    }

    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        access_token: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        self.request(reqwest::Method::POST, path, access_token, Some(body))
            .await
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
    ) -> Result<T, ApiError> {
        self.request(reqwest::Method::DELETE, path, access_token, None::<&()>)
            .await
    }

    async fn request<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        access_token: &str,
        body: Option<&B>,
    ) -> Result<T, ApiError> {
        let url = format!("{}/rest/v1/{}", self.base_url, path.trim_start_matches('/'));

        let mut request = self.http
            .request(method, url)
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .header("apikey", &self.anon_key)
            .header(CONTENT_TYPE, "application/json");

        if body.is_some() {
            request = request.header("Prefer", "return=representation").json(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::Supabase(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            tracing::error!("Supabase REST error {status}: {text}");
            return Err(ApiError::Supabase(text));
        }

        response
            .json()
            .await
            .map_err(|e| ApiError::Supabase(e.to_string()))
    }
}
impl SupabaseClient {
    /// Fetch a single backup by ID
    pub async fn get_backup_by_id(
        &self,
        backup_id: Uuid,
        user: &CurrentUser,
    ) -> Result<Option<Backup>, AppError> {
        let url = format!("{}/rest/v1/backups?id=eq.{}&select=*", self.url, backup_id);

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", user.access_token))
            .send()
            .await?;

        let backups: Vec<Backup> = response.json().await?;
        Ok(backups.into_iter().next())
    }

    /// Fetch a device by ID to confirm ownership
    pub async fn get_device_by_id(
        &self,
        device_id: Uuid,
        user: &CurrentUser,
    ) -> Result<Option<Device>, AppError> {
        let url = format!("{}/rest/v1/devices?id=eq.{}&select=*", self.url, device_id);

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", user.access_token))
            .send()
            .await?;

        let devices: Vec<Device> = response.json().await?;
        Ok(devices.into_iter().next())
    }

    /// Increment the fork count on a backup using RPC or PATCH
    pub async fn increment_fork_count(&self, backup_id: Uuid) -> Result<(), AppError> {
        let url = format!("{}/rest/v1/rpc/increment_fork_count", self.url);

        let _ = self
            .client
            .post(&url)
            .header("apikey", &self.anon_key)
            .json(&serde_json::json!({ "backup_id": backup_id }))
            .send()
            .await;

        Ok(())
    }
}
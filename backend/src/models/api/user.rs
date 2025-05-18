use crate::utils::response::{PaginatedResponse, Pagination};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub login_id: String,
    pub nickname: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login_id: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTempUserRequest {
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: i64,
    pub nickname: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: i64,
    pub nickname: String,
    pub last_active: Option<DateTime<Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance: f64,
    pub location_name: String,
    pub online_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserByLocationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserByNameRequest {
    pub keyword: String,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserByIdRequest {
    pub user_id: i64,
}

pub type SearchUserByLocationResponse = PaginatedResponse<UserInfo>;
pub type SearchUserByNameResponse = PaginatedResponse<UserInfo>;
pub type SearchUserByIdResponse = UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
}

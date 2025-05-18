use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// API请求模型
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
    pub password: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub group_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    pub group_id: Uuid,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGroupResponse {
    pub success: bool,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveGroupRequest {
    pub group_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveGroupResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KickMemberRequest {
    pub group_id: Uuid,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KickMemberResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub group_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupRequest {
    pub group_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupInfo {
    pub group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub creator_name: String,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub member_count: i64,
    pub distance: f64,
    pub location_name: String,
    pub is_password_required: bool,
    pub is_member: bool,
    pub user_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupResponse {
    pub group: GroupInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembersRequest {
    pub group_id: Uuid,
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMemberInfo {
    pub user_id: i64,
    pub nickname: String,
    pub last_active: String,
    pub role: String,
    pub join_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembersResponse {
    pub members: Vec<GroupMemberInfo>,
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchGroupByLocationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchGroupByNameRequest {
    pub keyword: String,
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchGroupByIdRequest {
    pub group_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchGroupByLocationResponse {
    pub groups: Vec<GroupInfo>,
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchGroupByNameResponse {
    pub groups: Vec<GroupInfo>,
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}

pub type SearchGroupByIdResponse = GroupInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOwnershipRequest {
    pub group_id: Uuid,
    pub new_owner_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOwnershipResponse {
    pub success: bool,
}

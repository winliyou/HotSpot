use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub data: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketMessageType {
    Ping,
    Pong,
    Connect,
    Connected,
    Error,
    Disconnect,
    DirectMessage,
    GroupMessage,
    UserOnline,
    UserOffline,
    UserTyping,
    ReadReceipt,
    GroupJoined,
    GroupLeft,
    NewCheckin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessageResponse {
    pub success: bool,
    pub message: Option<String>,
    pub message_type: WebSocketMessageType,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketConnectMessage {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectMessageEvent {
    pub message_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub recipient_id: String,
    pub content: String,
    pub message_type: String,
    pub sent_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMessageEvent {
    pub message_id: String,
    pub group_id: String,
    pub group_name: String,
    pub sender_id: i64,
    pub sender_name: String,
    pub content: String,
    pub message_type: String,
    pub sent_at: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatusEvent {
    pub user_id: String,
    pub status: String, // "online", "offline", "away"
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTypingEvent {
    pub user_id: i64,
    pub target_id: String,   // 会话ID或群组ID
    pub target_type: String, // "conversation" or "group"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadReceiptEvent {
    pub user_id: i64,
    pub conversation_id: String,
    pub last_read_message_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMembershipEvent {
    pub user_id: i64,
    pub nickname: String,
    pub group_id: String,
    pub group_name: String,
    pub event_type: String, // "joined" or "left"
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCheckinEvent {
    pub checkin_id: String,
    pub user_id: i64,
    pub nickname: String,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub preview: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupEventType {
    Create,  // 创建群组
    Join,    // 加入群组
    Leave,   // 离开群组
    Update,  // 更新群组信息
    Kick,    // 踢出成员
    Transfer, // 转让群主
}

impl std::fmt::Display for GroupEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupEventType::Create => write!(f, "create"),
            GroupEventType::Join => write!(f, "join"),
            GroupEventType::Leave => write!(f, "leave"),
            GroupEventType::Update => write!(f, "update"),
            GroupEventType::Kick => write!(f, "kick"),
            GroupEventType::Transfer => write!(f, "transfer"),
        }
    }
}

impl From<&str> for GroupEventType {
    fn from(s: &str) -> Self {
        match s {
            "create" => GroupEventType::Create,
            "join" => GroupEventType::Join,
            "leave" => GroupEventType::Leave,
            "update" => GroupEventType::Update,
            "kick" => GroupEventType::Kick,
            "transfer" => GroupEventType::Transfer,
            _ => GroupEventType::Create, // 默认值
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupEvent {
    pub group_id: String,
    pub event_type: GroupEventType,
    pub user_id: i64,
    pub user_nickname: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

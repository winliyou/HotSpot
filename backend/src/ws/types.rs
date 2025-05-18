// WebSocket 消息类型、事件、协议常量等统一定义
use crate::models::websocket::{WebSocketMessage, WebSocketMessageType};
use serde::{Deserialize, Serialize};

// 业务相关消息体可继续扩展
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPayload {
    pub sender_id: i64,
    pub content: String,
    pub timestamp: i64,
}

// ...其他 payload struct

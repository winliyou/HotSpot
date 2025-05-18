use crate::{middleware::auth::AppState, models::websocket::WebSocketConnectMessage};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tracing::{error, info};
use crate::models::websocket::{WebSocketMessage, WebSocketMessageType};
use chrono::Utc;

// WebSocket连接处理函数
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// 处理WebSocket连接
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // 等待认证消息
    let message = match receiver.next().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Ok(_)) => {
            let error_msg = serde_json::to_string(&WebSocketMessage {
                message_type: WebSocketMessageType::Error,
                data: Some(serde_json::json!({"error": "需要文本消息进行认证"})),
                timestamp: Utc::now(),
            }).unwrap();
            let _ = sender.send(Message::Text(error_msg.into())).await;
            return;
        }
        Some(Err(e)) => {
            error!("接收消息错误: {:?}", e);
            return;
        }
        None => {
            info!("连接关闭，未收到认证消息");
            return;
        }
    };

    // 解析连接消息
    let connect_msg: WebSocketConnectMessage = match serde_json::from_str(&message) {
        Ok(msg) => msg,
        Err(e) => {
            error!("无效的连接消息格式: {:?}", e);
            let error_msg = serde_json::to_string(&WebSocketMessage {
                message_type: WebSocketMessageType::Error,
                data: Some(serde_json::json!({"error": "无效的连接消息格式"})),
                timestamp: Utc::now(),
            }).unwrap();
            let _ = sender.send(Message::Text(error_msg.into())).await;
            return;
        }
    };

    // 提取并验证用户ID
    let user_id = match state.ws_service.verify_token(&connect_msg.token) {
        Ok(id) => id,
        Err(e) => {
            error!("Token验证失败: {}", e);
            let error_msg = serde_json::to_string(&WebSocketMessage {
                message_type: WebSocketMessageType::Error,
                data: Some(serde_json::json!({"error": format!("认证失败: {}", e)})),
                timestamp: Utc::now(),
            }).unwrap();
            let _ = sender.send(Message::Text(error_msg.into())).await;
            return;
        }
    };

    // 获取用户详细信息
    let user = match state.user_service.get_auth_info(user_id).await {
        Ok(user) => user,
        Err(e) => {
            error!("获取用户信息失败: {:?}", e);
            let error_msg = serde_json::to_string(&WebSocketMessage {
                message_type: WebSocketMessageType::Error,
                data: Some(serde_json::json!({"error": "获取用户信息失败"})),
                timestamp: Utc::now(),
            }).unwrap();
            let _ = sender.send(Message::Text(error_msg.into())).await;
            return;
        }
    };

    // 认证通过，进入主消息循环
    state.ws_service.handle_ws_stream(sender, receiver, user.user_id, user.nickname).await;
}

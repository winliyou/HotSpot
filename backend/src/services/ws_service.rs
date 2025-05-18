use crate::{
    models::websocket::{
        DirectMessageEvent, GroupMessageEvent, NewCheckinEvent, UserStatusEvent, WebSocketMessage,
        WebSocketMessageType,
    },
    services::ConfigService,
    utils::{jwt::verify_jwt_token, response::AppError},
    ws::session::SessionManager,
};
use axum::extract::ws::{Message, WebSocket};
use chrono::Utc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WsService {
    pub session_manager: SessionManager,
    pub tx: Sender<WebSocketMessage>,
    pub config_service: Arc<ConfigService>,
}

impl WsService {
    pub fn new(session_manager: SessionManager, config_service: Arc<ConfigService>) -> Self {
        // 创建一个广播通道，容量100
        let (tx, _) = broadcast::channel(100);

        Self {
            session_manager,
            tx,
            config_service,
        }
    }

    // 广播消息给所有连接的客户端
    pub fn broadcast_message(&self, message: WebSocketMessage) -> Result<(), String> {
        match self.tx.send(message.clone()) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("广播消息失败: {:?}", e);
                Err(format!("广播消息失败: {:?}", e))
            }
        }
    }

    // 创建新的订阅者
    pub fn subscribe(&self) -> Receiver<WebSocketMessage> {
        self.tx.subscribe()
    }

    // =============== 具体消息类型发送方法 ===============

    // 发送私信消息
    pub async fn send_direct_message(
        &self,
        message_id: Uuid,
        conversation_id: Uuid,
        sender_id: i64,
        sender_name: String,
        recipient_id: i64,
        content: String,
        message_type: String,
        latitude: f64,
        longitude: f64,
    ) -> Result<(), AppError> {
        let event = DirectMessageEvent {
            message_id: message_id.to_string(),
            conversation_id: conversation_id.to_string(),
            sender_id: sender_id.to_string(),
            sender_name,
            recipient_id: recipient_id.to_string(),
            content,
            message_type,
            sent_at: Utc::now(),
            latitude,
            longitude,
        };

        let ws_message = WebSocketMessage {
            message_type: WebSocketMessageType::DirectMessage,
            data: Some(serde_json::to_value(event).unwrap_or_default()),
            timestamp: Utc::now(),
        };

        self.tx.send(ws_message).map_err(|e| {
            error!("发送私信消息失败: {:?}", e);
            AppError::InternalServerError("发送消息失败".to_string())
        })?;

        Ok(())
    }

    // 发送群组消息
    pub async fn send_group_message(
        &self,
        message_id: &str,
        group_id: &str,
        group_name: &str,
        sender_id: i64,
        sender_nickname: &str,
        content: &str,
        content_type: &str,
    ) -> Result<(), AppError> {
        let event = GroupMessageEvent {
            message_id: message_id.to_string(),
            group_id: group_id.to_string(),
            group_name: group_name.to_string(),
            sender_id,
            sender_name: sender_nickname.to_string(),
            content: content.to_string(),
            message_type: content_type.to_string(),
            sent_at: Utc::now(),
            latitude: 0.0,
            longitude: 0.0,
        };

        let ws_message = WebSocketMessage {
            message_type: WebSocketMessageType::GroupMessage,
            data: Some(serde_json::to_value(event).unwrap_or_default()),
            timestamp: Utc::now(),
        };

        self.tx.send(ws_message).map_err(|e| {
            error!("发送群组消息失败: {:?}", e);
            AppError::InternalServerError("发送消息失败".to_string())
        })?;

        Ok(())
    }

    // 发送签到通知
    pub async fn send_checkin_notification(
        &self,
        checkin_id: &str,
        user_id: i64,
        nickname: &str,
        latitude: f64,
        longitude: f64,
        location_name: &str,
        preview: Option<&str>,
    ) -> Result<(), AppError> {
        let event = NewCheckinEvent {
            checkin_id: checkin_id.to_string(),
            user_id,
            nickname: nickname.to_string(),
            latitude,
            longitude,
            location_name: location_name.to_string(),
            preview: preview.map(|s| s.to_string()),
            timestamp: Utc::now(),
        };

        let ws_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewCheckin,
            data: Some(serde_json::to_value(event).unwrap_or_default()),
            timestamp: Utc::now(),
        };

        self.tx.send(ws_message).map_err(|e| {
            error!("发送签到通知失败: {:?}", e);
            AppError::InternalServerError("发送通知失败".to_string())
        })?;

        Ok(())
    }

    // 发送用户状态更新
    pub async fn send_user_status(&self, user_id: i64, status: &str) -> Result<(), AppError> {
        let event = UserStatusEvent {
            user_id: user_id.to_string(),
            status: status.to_string(),
            last_active: Utc::now(),
        };

        let ws_message = WebSocketMessage {
            message_type: WebSocketMessageType::UserOnline,
            data: Some(serde_json::to_value(event).unwrap_or_default()),
            timestamp: Utc::now(),
        };

        self.tx.send(ws_message).map_err(|e| {
            error!("发送用户状态更新失败: {:?}", e);
            AppError::InternalServerError("发送状态更新失败".to_string())
        })?;

        Ok(())
    }

    // =============== WebSocket连接处理 ===============

    // 处理已经分离的WebSocket流
    pub async fn handle_ws_stream(
        &self,
        mut sender: futures::stream::SplitSink<WebSocket, Message>,
        mut receiver: futures::stream::SplitStream<WebSocket>,
        user_id: i64,
        nickname: String,
    ) {
        // 添加会话
        self.session_manager
            .add_session(user_id, nickname.clone())
            .await;

        // 订阅广播消息
        let mut rx = self.tx.subscribe();

        // 发送认证成功消息
        let success_msg = json!({
            "type": "connected",
            "user_id": user_id,
            "timestamp": Utc::now().to_rfc3339()
        })
        .to_string();

        if let Err(e) = sender.send(Message::Text(success_msg.into())).await {
            error!("发送认证成功消息失败: {:?}", e);
            self.session_manager.remove_session(user_id).await;
            return;
        }

        // 广播用户上线消息
        if let Err(e) = self.send_user_status(user_id, "online").await {
            error!("广播用户上线消息失败: {:?}", e);
        }

        // 处理消息接收和发送
        loop {
            tokio::select! {
                // 接收消息
                Some(result) = receiver.next() => {
                    match result {
                        Ok(Message::Text(text)) => {
                            if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                                match ws_message.message_type {
                                    WebSocketMessageType::Ping => {
                                        let pong_msg = json!({
                                            "type": "pong",
                                            "timestamp": Utc::now().to_rfc3339()
                                        }).to_string();
                                        if let Err(e) = sender.send(Message::Text(pong_msg.into())).await {
                                            error!("发送pong消息失败: {:?}", e);
                                            break;
                                        }
                                    }
                                    WebSocketMessageType::Disconnect => {
                                        info!("用户 {} 请求断开连接", user_id);
                                        break;
                                    }
                                    _ => {
                                        // 更新会话活动时间
                                        self.session_manager.update_session_activity(user_id).await;

                                        // 广播消息给所有订阅者
                                        if let Err(e) = self.broadcast_message(ws_message) {
                                            error!("广播消息失败: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Message::Close(_)) => {
                            info!("用户 {} 关闭了连接", user_id);
                            break;
                        }
                        Ok(Message::Ping(data)) => {
                            if let Err(e) = sender.send(Message::Pong(data)).await {
                                error!("发送pong消息失败: {:?}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("接收消息错误: {:?}", e);
                            break;
                        }
                        _ => {}
                    }
                }
                // 接收广播消息
                Ok(message) = rx.recv() => {
                    let message_str = serde_json::to_string(&message).unwrap_or_default();
                    if let Err(e) = sender.send(Message::Text(message_str.into())).await {
                        error!("发送广播消息失败: {:?}", e);
                        break;
                    }
                }
                else => break
            }
        }

        // 用户断开连接，移除会话
        self.session_manager.remove_session(user_id).await;

        // 广播用户离线消息
        if let Err(e) = self.send_user_status(user_id, "offline").await {
            error!("广播用户离线消息失败: {:?}", e);
        }

        info!("用户 {} 已断开连接", user_id);
    }

    // 处理WebSocket连接
    pub async fn handle_socket(&self, socket: WebSocket, user_id: i64, nickname: String) {
        let (sender, receiver) = socket.split();
        self.handle_ws_stream(sender, receiver, user_id, nickname)
            .await;
    }

    // 验证JWT令牌
    pub fn verify_token(&self, token: &str) -> Result<i64, String> {
        let config = self.config_service.get_config();
        let claims = verify_jwt_token(token, &config.jwt_secret)
            .map_err(|_| "无效的token".to_string())?;
        Ok(claims.sub)
    }
}

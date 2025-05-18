use std::sync::Arc;

use axum::{Extension, Json, debug_handler, extract::State};

use crate::{
    middleware::auth::AppState,
    models::api::message::{
        ConversationsRequest, ConversationsResponse, DeleteDirectMessageRequest,
        DeleteGroupMessageRequest, DeleteMessageResponse, DirectMessageHistoryRequest,
        DirectMessageHistoryResponse, GroupMessageHistoryRequest, GroupMessageHistoryResponse,
        MarkReadRequest, MarkReadResponse, SendDirectMessageRequest, SendDirectMessageResponse,
        SendGroupMessageRequest, SendGroupMessageResponse,
    },
    models::api::user::AuthUser,
    utils::response::{ApiResponse, AppError, success_response},
    validators::{common_validator, message_validator},
};

// 发送群组文本消息
#[debug_handler]
pub async fn send_group_message(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendGroupMessageRequest>,
) -> Result<Json<ApiResponse<SendGroupMessageResponse>>, AppError> {
    // 验证消息内容
    message_validator::validate_message_content(&payload.content, &payload.message_type)?;

    // 验证位置参数（如果提供了位置信息）
    if payload.latitude.abs() > 0.0 || payload.longitude.abs() > 0.0 {
        common_validator::validate_location_params(payload.latitude, payload.longitude)?;
    }

    // 验证群组ID
    let group_id = common_validator::validate_uuid(&payload.group_id, "群组ID")?;

    // 检查用户是否在群组中
    if !state
        .message_service
        .check_user_in_group(group_id, current_user.user_id)
        .await?
    {
        return Err(AppError::BadRequest("您不是该群组成员".to_string()));
    }

    let send_group_message_result = state
        .message_service
        .send_group_message(
            group_id,
            current_user.user_id,
            &payload.content,
            &payload.message_type,
            payload.latitude,
            payload.longitude,
        )
        .await?;

    // 获取群组名称
    let group = state
        .group_service
        .search_group_by_id(current_user.user_id, group_id)
        .await?;

    // 通过WebSocket发送实时消息
    let _ = state
        .ws_service
        .send_group_message(
            &send_group_message_result.message_id.to_string(),
            &payload.group_id,
            &group.name,
            current_user.user_id,
            &current_user.nickname,
            &payload.content,
            &payload.message_type,
        )
        .await;

    let response = SendGroupMessageResponse {
        group_id: payload.group_id.clone(),
        message_id: send_group_message_result.message_id.to_string(),
        message_type: payload.message_type.clone(),
        sent_at: chrono::Utc::now(),
    };
    Ok(success_response(response))
}

#[debug_handler]
pub async fn group_history(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GroupMessageHistoryRequest>,
) -> Result<Json<ApiResponse<GroupMessageHistoryResponse>>, AppError> {
    // 验证群组ID
    let group_id = common_validator::validate_uuid(&payload.group_id, "群组ID")?;

    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(20);

    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?; // 最大每页50条
    }

    let (messages, has_more, next_cursor) = state
        .message_service
        .get_group_messages(group_id, current_user.user_id, cursor, limit)
        .await?;

    let items = messages
        .into_iter()
        .map(|msg| crate::models::api::message::GroupMessageHistoryItem {
            message_id: msg.message_id.to_string(),
            group_id: msg.group_id.to_string(),
            sender_id: msg.sender_id.to_string(),
            sender_name: String::new(), // 可补充
            content: msg.content,
            message_type: msg.message_type,
            sent_at: msg.created_at,
            latitude: msg.latitude,
            longitude: msg.longitude,
        })
        .collect();

    let response = crate::utils::response::PaginatedResponse {
        items,
        pagination: crate::utils::response::PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    Ok(success_response(response))
}

// 发送私聊消息
#[debug_handler]
pub async fn send_direct_message(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendDirectMessageRequest>,
) -> Result<Json<ApiResponse<SendDirectMessageResponse>>, AppError> {
    // 验证消息内容
    message_validator::validate_message_content(&payload.content, &payload.message_type)?;

    // 验证位置参数（如果提供了位置信息）
    if payload.latitude.abs() > 0.0 || payload.longitude.abs() > 0.0 {
        common_validator::validate_location_params(payload.latitude, payload.longitude)?;
    }

    // 验证接收者ID
    let recipient_id = message_validator::validate_recipient_id(&payload.recipient_id)?;

    let message = state
        .message_service
        .send_direct_message(
            current_user.user_id,
            recipient_id,
            &payload.content,
            &payload.message_type,
            payload.latitude,
            payload.longitude,
        )
        .await?;

    // 通过WebSocket发送实时消息
    let _ = state
        .ws_service
        .send_direct_message(
            message.message_id,
            message.conversation_id,
            current_user.user_id,
            current_user.nickname.clone(),
            recipient_id,
            payload.content.clone(),
            payload.message_type.clone(),
            payload.latitude,
            payload.longitude,
        )
        .await;

    let response = SendDirectMessageResponse {
        message_id: message.message_id.to_string(),
        conversation_id: message.conversation_id.to_string(),
        recipient_id: payload.recipient_id.clone(),
        message_type: message.message_type.clone(),
        sent_at: message.created_at,
    };
    Ok(success_response(response))
}

// 获取私聊消息历史
#[debug_handler]
pub async fn user_history(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DirectMessageHistoryRequest>,
) -> Result<Json<ApiResponse<DirectMessageHistoryResponse>>, AppError> {
    // 验证会话ID
    message_validator::validate_conversation_id(payload.conversation_id.as_ref())?;

    let conversation_id =
        common_validator::validate_uuid(payload.conversation_id.as_ref().unwrap(), "会话ID")?;

    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(20);

    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?; // 最大每页50条
    }

    let (messages, has_more, next_cursor) = state
        .message_service
        .get_direct_messages(conversation_id, current_user.user_id, cursor, limit)
        .await?;

    let items = messages
        .into_iter()
        .map(
            |msg| crate::models::api::message::DirectMessageHistoryItem {
                message_id: msg.message_id.to_string(),
                conversation_id: msg.conversation_id.to_string(),
                sender_id: msg.sender_id.to_string(),
                sender_name: String::new(), // 可补充
                recipient_id: msg.recipient_id.to_string(),
                content: msg.content,
                message_type: msg.message_type,
                sent_at: msg.created_at,
                read_at: msg.read_at,
                latitude: msg.latitude,
                longitude: msg.longitude,
            },
        )
        .collect();

    let response = crate::utils::response::PaginatedResponse {
        items,
        pagination: crate::utils::response::PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    Ok(success_response(response))
}

// 获取会话列表
#[debug_handler]
pub async fn user_conversations(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ConversationsRequest>,
) -> Result<Json<ApiResponse<ConversationsResponse>>, AppError> {
    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?; // 最大每页50条
    }

    let conversations = state
        .message_service
        .get_conversations(current_user.user_id)
        .await?;

    let items = conversations
        .into_iter()
        .map(|conv| crate::models::api::message::ConversationItem {
            conversation_id: conv.conversation_id.to_string(),
            peer_id: if conv.user1_id == current_user.user_id {
                conv.user2_id
            } else {
                conv.user1_id
            },
            peer_name: String::new(), // 可补充
            last_message_preview: conv.last_message_preview.unwrap_or_default(),
            last_message_time: conv.last_message_at.unwrap_or(conv.created_at),
            unread_count: 0, // 可补充
        })
        .collect();

    let response = crate::utils::response::PaginatedResponse {
        items,
        pagination: crate::utils::response::PaginationMeta {
            has_more: false,   // 可补充
            next_cursor: None, // 可补充
        },
    };

    Ok(success_response(response))
}

// 删除群组消息
#[debug_handler]
pub async fn delete_group_message(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteGroupMessageRequest>,
) -> Result<Json<ApiResponse<DeleteMessageResponse>>, AppError> {
    let message_id = common_validator::validate_uuid(&payload.message_id, "消息ID")?;

    state
        .message_service
        .delete_group_message(message_id, current_user.user_id)
        .await?;

    Ok(success_response(DeleteMessageResponse { success: true }))
}

// 删除私聊消息
#[debug_handler]
pub async fn delete_direct_message(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteDirectMessageRequest>,
) -> Result<Json<ApiResponse<DeleteMessageResponse>>, AppError> {
    let message_id = common_validator::validate_uuid(&payload.message_id, "消息ID")?;

    state
        .message_service
        .delete_direct_message(message_id, current_user.user_id)
        .await?;

    Ok(success_response(DeleteMessageResponse { success: true }))
}

// 标记消息已读
#[debug_handler]
pub async fn mark_messages_read(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MarkReadRequest>,
) -> Result<Json<ApiResponse<MarkReadResponse>>, AppError> {
    // 会话ID是必须的
    if payload.conversation_id.is_empty() {
        return Err(AppError::BadRequest("必须提供会话ID".to_string()));
    }

    // 解析会话ID
    let conversation_id = common_validator::validate_uuid(&payload.conversation_id, "会话ID")?;

    // 解析消息ID（如果存在）
    let message_id = if let Some(msg_id) = payload.message_id {
        if msg_id.is_empty() {
            None
        } else {
            Some(common_validator::validate_uuid(&msg_id, "消息ID")?)
        }
    } else {
        None
    };

    // 调用service标记已读
    let marked_count = state
        .message_service
        .mark_messages_read(conversation_id, current_user.user_id, message_id)
        .await?;

    // 构建响应
    let response = MarkReadResponse {
        success: true,
        marked_count: Some(marked_count),
    };

    Ok(success_response(response))
}

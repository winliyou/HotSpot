use crate::utils::response::AppError;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GroupMessageRow {
    pub message_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub sender_id: i64,
    pub content: String,
    pub message_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

pub struct MessageService {
    pool: PgPool,
}

#[derive(Debug, Clone)]
pub struct MessageRow {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: i64,
    pub recipient_id: i64,
    pub content: String,
    pub message_type: String,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct ConversationRow {
    pub conversation_id: Uuid,
    pub user1_id: i64,
    pub user2_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub last_message_preview: Option<String>,
}

#[derive(Debug)]
pub struct MessageLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl MessageService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =============== 验证函数 ===============

    // 检查私信会话是否存在
    pub async fn check_conversation_exists(&self, conversation_id: Uuid) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM conversations WHERE conversation_id = $1) as "exists!""#,
            conversation_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("查询私信会话失败: {:?}", e);
            AppError::InternalServerError("查询私信会话失败".to_string())
        })?;

        Ok(exists)
    }

    // 检查用户是否在私信会话中
    pub async fn check_user_in_conversation(
        &self,
        conversation_id: Uuid,
        user_id: i64,
    ) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM conversations 
                WHERE conversation_id = $1 AND (user1_id = $2 OR user2_id = $2)
            ) as "exists!""#,
            conversation_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("检查用户是否在私信会话失败: {:?}", e);
            AppError::InternalServerError("检查用户私信会话失败".to_string())
        })?;

        Ok(exists)
    }

    // 检查用户是否在群组中
    pub async fn check_user_in_group(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM group_members 
                WHERE group_id = $1 AND user_id = $2
            ) as "exists!""#,
            group_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("检查用户是否在群组失败: {:?}", e);
            AppError::InternalServerError("检查用户群组失败".to_string())
        })?;

        Ok(exists)
    }

    // 检查群组是否存在
    pub async fn check_group_exists(&self, group_id: Uuid) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM groups WHERE group_id = $1) as "exists!""#,
            group_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("查询群组失败: {:?}", e);
            AppError::InternalServerError("查询群组失败".to_string())
        })?;

        Ok(exists)
    }

    // 获取私信会话中的另一个用户
    pub async fn get_other_user_in_conversation(
        &self,
        conversation_id: Uuid,
        current_user_id: i64,
    ) -> Result<i64, AppError> {
        let user1_id = sqlx::query_scalar!(
            r#"SELECT user1_id FROM conversations WHERE conversation_id = $1"#,
            conversation_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("获取私信会话信息失败: {:?}", e);
            AppError::NotFound("私信会话不存在".to_string())
        })?;

        let user2_id = sqlx::query_scalar!(
            r#"SELECT user2_id FROM conversations WHERE conversation_id = $1"#,
            conversation_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("获取私信会话信息失败: {:?}", e);
            AppError::NotFound("私信会话不存在".to_string())
        })?;

        let other_user_id = if user1_id == current_user_id {
            user2_id
        } else {
            user1_id
        };

        Ok(other_user_id)
    }

    // =============== 操作函数 ===============

    // 创建私信会话
    pub async fn create_dm_session(&self, user_1: i64, user_2: i64) -> Result<Uuid, AppError> {
        // 确保 user_1 总是较小的用户ID，保证一致性
        let (smaller_id, larger_id) = if user_1 < user_2 {
            (user_1, user_2)
        } else {
            (user_2, user_1)
        };

        // 首先查找是否已存在会话
        let existing_session = sqlx::query_scalar!(
            r#"
            SELECT conversation_id FROM conversations 
            WHERE (user1_id = $1 AND user2_id = $2) OR (user1_id = $2 AND user2_id = $1)
            "#,
            smaller_id,
            larger_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("查询私信会话失败: {:?}", e);
            AppError::InternalServerError("查询私信会话失败".to_string())
        })?;

        if let Some(session) = existing_session {
            return Ok(session);
        }

        // 创建新会话
        let conversation_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO conversations (conversation_id, user1_id, user2_id, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            conversation_id,
            smaller_id,
            larger_id,
            Utc::now()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("创建私信会话失败: {:?}", e);
            AppError::InternalServerError("创建私信会话失败".to_string())
        })?;

        Ok(conversation_id)
    }

    // 保存消息记录
    pub async fn save_direct_message(
        &self,
        conversation_id: Uuid,
        sender_id: i64,
        recipient_id: i64,
        content: &str,
        content_type: &str,
        latitude: f64,
        longitude: f64,
    ) -> Result<Uuid, AppError> {
        let message_id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO direct_messages (message_id, conversation_id, sender_id, recipient_id, content, message_type, created_at, latitude, longitude)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            message_id,
            conversation_id,
            sender_id,
            recipient_id,
            content,
            content_type,
            now,
            latitude,
            longitude
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("保存私信消息失败: {:?}", e);
            AppError::InternalServerError("保存私信消息失败".to_string())
        })?;
        Ok(message_id)
    }

    // 保存群聊消息
    pub async fn save_group_message(
        &self,
        group_id: Uuid,
        sender_id: i64,
        content: &str,
        message_type: &str,
        latitude: f64,
        longitude: f64,
    ) -> Result<Uuid, AppError> {
        let message_id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO group_messages (message_id, group_id, sender_id, content, message_type, created_at, latitude, longitude)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            message_id,
            group_id,
            sender_id,
            content,
            message_type,
            now,
            latitude,
            longitude
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("保存群聊消息失败: {:?}", e)))?;
        Ok(message_id)
    }

    // =============== 业务函数 ===============

    // 发送私信
    pub async fn send_direct_message(
        &self,
        sender_id: i64,
        recipient_id: i64,
        content: &str,
        message_type: &str,
        latitude: f64,
        longitude: f64,
    ) -> Result<MessageRow, AppError> {
        // 获取或创建会话
        let conversation_id = self
            .get_or_create_conversation(sender_id, recipient_id)
            .await?;

        // 保存消息
        let message_id = self
            .save_direct_message(
                conversation_id,
                sender_id,
                recipient_id,
                content,
                message_type,
                latitude,
                longitude,
            )
            .await?;

        // 获取消息详情
        let message = sqlx::query_as!(
            MessageRow,
            r#"
            SELECT 
                message_id as "message_id!",
                conversation_id as "conversation_id!",
                sender_id as "sender_id!",
                recipient_id as "recipient_id!",
                content as "content!",
                message_type as "message_type!",
                created_at as "created_at!",
                read_at,
                latitude as "latitude!: f64",
                longitude as "longitude!: f64"
            FROM direct_messages
            WHERE message_id = $1
            "#,
            message_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("获取消息详情失败: {:?}", e);
            AppError::InternalServerError("获取消息详情失败".to_string())
        })?;

        Ok(message)
    }

    // 发送群组消息
    pub async fn send_group_message(
        &self,
        group_id: Uuid,
        sender_id: i64,
        content: &str,
        message_type: &str,
        latitude: f64,
        longitude: f64,
    ) -> Result<GroupMessageRow, AppError> {
        // 检查用户是否在群组中
        if !self.check_user_in_group(group_id, sender_id).await? {
            return Err(AppError::BadRequest("您不是群组成员".to_string()));
        }

        // 保存消息
        let message_id = self
            .save_group_message(
                group_id,
                sender_id,
                content,
                message_type,
                latitude,
                longitude,
            )
            .await?;

        // 获取消息详情
        let message = sqlx::query_as!(
            GroupMessageRow,
            r#"
            SELECT 
                message_id as "message_id!",
                group_id as "group_id!",
                sender_id as "sender_id!",
                content as "content!",
                message_type as "message_type!",
                created_at as "created_at!",
                latitude as "latitude!: f64",
                longitude as "longitude!: f64"
            FROM group_messages
            WHERE message_id = $1
            "#,
            message_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("获取消息详情失败: {:?}", e);
            AppError::InternalServerError("获取消息详情失败".to_string())
        })?;

        Ok(message)
    }

    pub async fn create_dm_session_with_user(
        &self,
        user_id: i64,
        current_user_id: i64,
    ) -> Result<Uuid, AppError> {
        // 创建或获取会话ID
        let conversation_id = self.create_dm_session(current_user_id, user_id).await?;

        Ok(conversation_id)
    }

    // 获取或创建私信会话
    pub async fn get_or_create_conversation(
        &self,
        user1_id: i64,
        user2_id: i64,
    ) -> Result<Uuid, AppError> {
        let (smaller_id, larger_id) = if user1_id < user2_id {
            (user1_id, user2_id)
        } else {
            (user2_id, user1_id)
        };
        let existing_conv = sqlx::query_scalar!(
            r#"
            SELECT conversation_id FROM conversations 
            WHERE (user1_id = $1 AND user2_id = $2) OR (user1_id = $2 AND user2_id = $1)
            "#,
            smaller_id,
            larger_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("查询会话失败: {:?}", e)))?;
        if let Some(conv) = existing_conv {
            return Ok(conv);
        }
        let conversation_id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO conversations (conversation_id, user1_id, user2_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            "#,
            conversation_id,
            smaller_id,
            larger_id,
            now
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("插入会话失败: {:?}", e)))?;
        Ok(conversation_id)
    }

    // 获取私信会话列表
    pub async fn get_conversations(&self, user_id: i64) -> Result<Vec<ConversationRow>, AppError> {
        let conversations = sqlx::query_as!(
            ConversationRow,
            r#"
            SELECT 
                conversation_id as "conversation_id!",
                user1_id as "user1_id!",
                user2_id as "user2_id!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                last_message_at,
                last_message_preview
            FROM conversations
            WHERE user1_id = $1 OR user2_id = $1
            ORDER BY last_message_at DESC NULLS LAST
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("获取会话列表失败: {:?}", e)))?;
        Ok(conversations)
    }

    // 获取私信消息列表（游标分页，规范实现）
    pub async fn get_direct_messages(
        &self,
        conversation_id: Uuid,
        user_id: i64,
        cursor: i64,
        limit: i64,
    ) -> Result<(Vec<MessageRow>, bool, Option<i64>), AppError> {
        // 验证用户是否在会话中
        let is_member = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM conversations
                WHERE conversation_id = $1
                AND (user1_id = $2 OR user2_id = $2)
            ) as "exists!"
            "#,
            conversation_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("验证用户会话失败: {:?}", e)))?;

        if !is_member {
            return Err(AppError::Forbidden("User not in conversation".into()));
        }

        let rows = sqlx::query_as!(
            MessageRow,
            r#"
            SELECT 
                message_id as "message_id!",
                conversation_id as "conversation_id!",
                sender_id as "sender_id!",
                recipient_id as "recipient_id!",
                content as "content!",
                message_type as "message_type!",
                created_at as "created_at!",
                read_at,
                latitude as "latitude!: f64",
                longitude as "longitude!: f64"
            FROM direct_messages
            WHERE conversation_id = $1
            AND ($2::bigint = 0 OR id < $2::bigint)
            ORDER BY id DESC
            LIMIT $3
            "#,
            conversation_id,
            cursor,
            limit + 1
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("获取私信消息失败: {:?}", e)))?;

        // 标记消息为已读
        sqlx::query!(
            r#"
            UPDATE direct_messages
            SET read_at = CURRENT_TIMESTAMP
            WHERE conversation_id = $1
            AND recipient_id = $2
            AND read_at IS NULL
            "#,
            conversation_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("标记已读失败: {:?}", e)))?;

        let has_more = rows.len() > limit as usize;
        let items = if has_more {
            rows[..limit as usize].to_vec()
        } else {
            rows
        };
        let next_cursor = if has_more && !items.is_empty() {
            // 取最后一条的 id 作为游标
            let last_msg = items.last().unwrap();
            // 需要查 id 字段
            sqlx::query_scalar!(
                "SELECT id FROM direct_messages WHERE message_id = $1",
                last_msg.message_id
            )
            .fetch_one(&self.pool)
            .await
            .ok()
        } else {
            None
        };
        Ok((items, has_more, next_cursor))
    }

    // 获取群组消息
    pub async fn get_group_messages(
        &self,
        group_id: Uuid,
        _user_id: i64,
        cursor: i64,
        limit: i64,
    ) -> Result<(Vec<GroupMessageRow>, bool, Option<i64>), AppError> {
        // 使用单一SQL语句，cursor参数始终传入一个值
        let messages = sqlx::query_as!(
            GroupMessageRow,
            r#"
            SELECT 
                message_id as "message_id!",
                group_id as "group_id!",
                sender_id as "sender_id!",
                content as "content!",
                message_type as "message_type!",
                created_at as "created_at!",
                latitude as "latitude!: f64",
                longitude as "longitude!: f64"
            FROM group_messages
            WHERE group_id = $1 
            AND id < $2
            ORDER BY created_at DESC
            LIMIT $3
            "#,
            group_id,
            cursor,
            limit + 1
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("获取群组消息失败: {:?}", e);
            AppError::InternalServerError("获取群组消息失败".to_string())
        })?;

        let has_more = messages.len() > limit as usize;
        let messages_limited = if has_more {
            messages[..limit as usize].to_vec()
        } else {
            messages
        };

        // 使用最后一条消息的 id 作为游标
        let next_cursor = if has_more && !messages_limited.is_empty() {
            // 查询最后一条消息的 id
            let last_msg = messages_limited.last().unwrap();
            sqlx::query_scalar!(
                "SELECT id FROM group_messages WHERE message_id = $1",
                last_msg.message_id
            )
            .fetch_one(&self.pool)
            .await
            .ok()
        } else {
            None
        };

        Ok((messages_limited, has_more, next_cursor))
    }

    // 标记消息已读
    pub async fn mark_messages_read(
        &self,
        conversation_id: Uuid,
        user_id: i64,
        message_id: Option<Uuid>,
    ) -> Result<i64, AppError> {
        // 验证用户是否在会话中
        let is_member = self
            .check_user_in_conversation(conversation_id, user_id)
            .await?;
        if !is_member {
            return Err(AppError::Forbidden("您不是该会话的成员".to_string()));
        }

        if let Some(msg_id) = message_id {
            // 验证消息是否存在于该会话
            let msg_exists = sqlx::query_scalar!(
                r#"SELECT EXISTS(SELECT 1 FROM direct_messages 
                   WHERE message_id = $1 AND conversation_id = $2) as "exists!""#,
                msg_id,
                conversation_id
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(format!("验证消息存在失败: {:?}", e)))?;

            if !msg_exists {
                return Err(AppError::NotFound("指定的消息不存在于该会话".to_string()));
            }

            // 只标记该消息之前（含该消息）的所有消息为已读
            let result = sqlx::query!(
                r#"
                UPDATE direct_messages 
                SET read_at = CURRENT_TIMESTAMP 
                WHERE conversation_id = $1 
                AND recipient_id = $2 
                AND read_at IS NULL
                AND created_at <= (SELECT created_at FROM direct_messages WHERE message_id = $3)
                "#,
                conversation_id,
                user_id,
                msg_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(format!("标记消息已读失败: {:?}", e)))?;

            Ok(result.rows_affected() as i64)
        } else {
            // 标记会话中所有未读消息为已读
            let result = sqlx::query!(
                r#"
                UPDATE direct_messages 
                SET read_at = CURRENT_TIMESTAMP 
                WHERE conversation_id = $1 
                AND recipient_id = $2 
                AND read_at IS NULL
                "#,
                conversation_id,
                user_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(format!("标记消息已读失败: {:?}", e)))?;

            Ok(result.rows_affected() as i64)
        }
    }

    // 删除群组消息
    pub async fn delete_group_message(
        &self,
        message_id: Uuid,
        user_id: i64,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"DELETE FROM group_messages WHERE message_id = $1 AND sender_id = $2"#,
            message_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("删除群组消息失败: {:?}", e)))?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("消息不存在或无权删除".to_string()));
        }
        Ok(())
    }

    // 删除私聊消息
    pub async fn delete_direct_message(
        &self,
        message_id: Uuid,
        user_id: i64,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"DELETE FROM direct_messages WHERE message_id = $1 AND sender_id = $2"#,
            message_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("删除私聊消息失败: {:?}", e)))?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("消息不存在或无权删除".to_string()));
        }
        Ok(())
    }
}

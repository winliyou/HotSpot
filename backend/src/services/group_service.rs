use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{
        db::Database,
        location::{DEFAULT_LATITUDE, DEFAULT_LONGITUDE},
    },
    models::constants::group_roles,
    utils::response::AppError,
};

// =============== 请求参数结构体 ===============

#[derive(Debug, Deserialize)]
pub struct GroupCreateParams {
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GroupSearchParams {
    pub user_id: i64,
    pub keyword: String,
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GroupLocationSearchParams {
    pub user_id: i64,
    pub radius: f64,
    pub cursor: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GroupJoinParams {
    pub group_id: Uuid,
    pub password: Option<String>,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GroupLeaveParams {
    pub group_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GroupKickParams {
    pub group_id: Uuid,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GroupTransferParams {
    pub group_id: Uuid,
    pub new_owner_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GroupIdSearchParams {
    pub user_id: i64,
    pub group_id: Uuid,
}

// =============== 返回结构体 ===============

#[derive(Debug, Clone, Serialize)]
pub struct GroupRow {
    pub id: i64,
    pub group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub creator_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_active_at: chrono::DateTime<chrono::Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub member_count: i64,
    pub distance: f64,
    pub is_password_required: bool,
    pub is_member: bool,
    pub user_role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupMemberRow {
    pub id: i64,
    pub user_id: i64,
    pub nickname: Option<String>,
    pub last_active: Option<String>,
    pub role: Option<String>,
    pub join_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct GroupMemberList {
    pub members: Vec<GroupMemberRow>,
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GroupActionResult {
    pub success: bool,
    pub role: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GroupService {
    db: Database,
}

impl GroupService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // =============== 验证函数 ===============

    // 检查群组是否存在
    pub async fn check_group_exists(&self, group_id: Uuid) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM groups WHERE group_id = $1) as "exists!""#,
            group_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("查询群组失败: {:?}", e);
            AppError::InternalServerError("查询群组失败".to_string())
        })?;

        Ok(exists)
    }

    // 检查用户是否是群主
    pub async fn check_user_is_group_owner(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<bool, AppError> {
        let is_owner = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM groups 
                WHERE group_id = $1 AND owner_id = $2
            ) as "exists!""#,
            group_id,
            user_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("检查用户是否是群主失败: {:?}", e);
            AppError::InternalServerError("检查群组所有权失败".to_string())
        })?;

        Ok(is_owner)
    }

    // 检查用户是否在群组中
    pub async fn check_user_in_group(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<bool, AppError> {
        let is_member = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM group_members 
                WHERE group_id = $1 AND user_id = $2
            ) as "exists!""#,
            group_id,
            user_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("检查用户是否在群组中失败: {:?}", e);
            AppError::InternalServerError("检查群组成员失败".to_string())
        })?;

        Ok(is_member)
    }

    // 检查用户是否是群组管理员
    pub async fn check_user_is_group_admin(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<bool, AppError> {
        // 先检查是否是群主
        if self.check_user_is_group_owner(group_id, user_id).await? {
            return Ok(true);
        }

        // 检查是否是管理员
        let is_admin = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM group_members 
                WHERE group_id = $1 AND user_id = $2 AND role = $3
            ) as "exists!""#,
            group_id,
            user_id,
            group_roles::ADMIN
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("检查用户是否是群组管理员失败: {:?}", e);
            AppError::InternalServerError("检查群组管理员失败".to_string())
        })?;

        Ok(is_admin)
    }

    // 获取群组密码哈希
    pub async fn get_group_password_hash(
        &self,
        group_id: Uuid,
    ) -> Result<Option<String>, AppError> {
        let password_hash = sqlx::query_scalar!(
            r#"SELECT password_hash FROM groups WHERE group_id = $1"#,
            group_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("获取群组密码哈希失败: {:?}", e);
            AppError::InternalServerError("获取群组密码哈希失败".to_string())
        })?;

        Ok(password_hash)
    }

    // 检查群组是否需要密码
    pub async fn check_group_need_password(&self, group_id: Uuid) -> Result<bool, AppError> {
        let need_password = sqlx::query_scalar!(
            r#"SELECT (password_hash IS NOT NULL) as "need_password!" FROM groups WHERE group_id = $1"#,
            group_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("检查群组密码需求失败: {:?}", e);
            AppError::InternalServerError("检查群组信息失败".to_string())
        })?;

        Ok(need_password)
    }

    // 验证密码是否正确
    pub async fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| {
            tracing::error!("解析密码哈希失败: {:?}", e);
            AppError::InternalServerError("密码验证失败".to_string())
        })?;

        let result = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(result)
    }

    // 验证群组密码
    pub async fn verify_group_password(
        &self,
        group_id: Uuid,
        password: &str,
    ) -> Result<bool, AppError> {
        // 获取群组密码哈希
        let password_hash = match self.get_group_password_hash(group_id).await? {
            Some(hash) => hash,
            None => return Ok(true), // 如果群组没有设置密码，直接返回验证成功
        };

        let parsed_hash = PasswordHash::new(&password_hash).map_err(|e| {
            tracing::error!("解析密码哈希失败: {:?}", e);
            AppError::InternalServerError("密码验证失败".to_string())
        })?;

        let result = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(result)
    }

    // 获取群组管理员数量
    pub async fn count_group_admins(&self, group_id: Uuid) -> Result<i64, AppError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM group_members WHERE group_id = $1 AND role = $2",
            group_id,
            group_roles::ADMIN
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("查询群组管理员数量失败: {:?}", e);
            AppError::InternalServerError("查询群组信息失败".to_string())
        })?
        .unwrap_or(0);

        Ok(count)
    }

    // 获取群组成员数量
    pub async fn count_group_members(&self, group_id: Uuid) -> Result<i64, AppError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM group_members WHERE group_id = $1",
            group_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("查询群组成员数量失败: {:?}", e);
            AppError::InternalServerError("查询群组信息失败".to_string())
        })?
        .unwrap_or(0);

        Ok(count)
    }

    // =============== 操作函数 ===============

    // 哈希密码
    pub async fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                tracing::error!("密码加密失败: {:?}", e);
                AppError::InternalServerError("密码处理失败".to_string())
            })?
            .to_string();

        Ok(password_hash)
    }

    // 创建群组（事务操作）
    pub async fn create_group(
        &self,
        name: &str,
        description: Option<&str>,
        owner_id: i64,
        latitude: f64,
        longitude: f64,
        location_name: &str,
        password: Option<&str>,
    ) -> Result<Uuid, AppError> {
        let group_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        // 开启事务
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            tracing::error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 处理密码
        let password_hash = if let Some(pwd) = password {
            Some(self.hash_password(pwd).await?)
        } else {
            None
        };

        // 创建群组
        sqlx::query!(
            r#"
            INSERT INTO groups (
                group_id, name, description, owner_id, 
                latitude, longitude, location_name, password_hash,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            group_id,
            name,
            description,
            owner_id,
            latitude,
            longitude,
            location_name,
            password_hash,
            now,
            now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("创建群组失败: {:?}", e);
            AppError::InternalServerError("创建群组失败".to_string())
        })?;

        // 添加群主为成员
        sqlx::query!(
            r#"
            INSERT INTO group_members (group_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, $4)
            "#,
            group_id,
            owner_id,
            group_roles::OWNER,
            now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("添加群主为成员失败: {:?}", e);
            AppError::InternalServerError("创建群组失败".to_string())
        })?;

        // 提交事务
        tx.commit().await.map_err(|e| {
            tracing::error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(group_id)
    }

    // =============== 业务函数 ===============

    // 加入群组
    pub async fn join_group(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<GroupActionResult, AppError> {
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            tracing::error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;
        sqlx::query!(
            r#"
            INSERT INTO group_members (group_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, $4)
            "#,
            group_id,
            user_id,
            group_roles::MEMBER,
            chrono::Utc::now(),
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("添加群组成员失败: {:?}", e);
            AppError::InternalServerError("加入群组失败".to_string())
        })?;
        tx.commit().await.map_err(|e| {
            tracing::error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(GroupActionResult {
            success: true,
            role: Some(group_roles::MEMBER.to_string()),
        })
    }

    pub async fn leave_group(
        &self,
        group_id: Uuid,
        user_id: i64,
    ) -> Result<GroupActionResult, AppError> {
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            tracing::error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        sqlx::query!(
            r#"DELETE FROM group_members WHERE group_id = $1 AND user_id = $2"#,
            group_id,
            user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("移除群组成员失败: {:?}", e);
            AppError::InternalServerError("离开群组失败".to_string())
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(GroupActionResult {
            success: true,
            role: None,
        })
    }

    pub async fn kick_member(
        &self,
        group_id: Uuid,
        user_id: i64,
        admin_id: i64,
    ) -> Result<GroupActionResult, AppError> {
        if admin_id == user_id {
            return Err(AppError::BadRequest(
                "无法踢出自己，请使用离开群组".to_string(),
            ));
        }

        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            tracing::error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        sqlx::query!(
            r#"DELETE FROM group_members WHERE group_id = $1 AND user_id = $2"#,
            group_id,
            user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("移除群组成员失败: {:?}", e);
            AppError::InternalServerError("踢出成员失败".to_string())
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(GroupActionResult {
            success: true,
            role: None,
        })
    }

    pub async fn transfer_ownership(
        &self,
        group_id: Uuid,
        new_owner_id: i64,
        user_id: i64,
    ) -> Result<GroupActionResult, AppError> {
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            tracing::error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        sqlx::query!(
            r#"
            UPDATE groups 
            SET owner_id = $1, updated_at = $2
            WHERE group_id = $3
            "#,
            new_owner_id,
            chrono::Utc::now(),
            group_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("更新群主失败: {:?}", e);
            AppError::InternalServerError("转让群主失败".to_string())
        })?;

        sqlx::query!(
            r#"
            UPDATE group_members 
            SET role = CASE 
                WHEN user_id = $1 THEN $4
                WHEN user_id = $2 THEN $5
                ELSE role
            END
            WHERE group_id = $3 AND user_id IN ($1, $2)
            "#,
            new_owner_id,
            user_id,
            group_id,
            group_roles::OWNER,
            group_roles::MEMBER
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("更新成员角色失败: {:?}", e);
            AppError::InternalServerError("转让群主失败".to_string())
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(GroupActionResult {
            success: true,
            role: None,
        })
    }

    // 获取群组成员列表
    pub async fn get_group_members(
        &self,
        group_id: Uuid,
        cursor: i64,
        limit: i64,
        user_id: i64,
    ) -> Result<GroupMemberList, AppError> {
        // 先检查用户是否属于该群组
        let is_member = self.check_user_in_group(group_id, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden("您不是该群组的成员".to_string()));
        }

        // 获取成员列表
        let rows = sqlx::query_as!(
            GroupMemberRow,
            r#"
            SELECT 
                gm.id as "id!",
                gm.user_id as "user_id!",
                u.nickname as nickname,
                CASE 
                    WHEN u.last_active_at > NOW() - INTERVAL '5 minutes' THEN 'online'
                    ELSE 'offline'
                END as last_active,
                gm.role::text as role,
                gm.joined_at as join_time
            FROM group_members gm
            JOIN users u ON gm.user_id = u.user_id
            WHERE gm.group_id = $1
            AND ($2::bigint = 0 OR gm.id < $2::bigint)
            ORDER BY gm.id DESC
            LIMIT $3
            "#,
            group_id,
            cursor,
            limit + 1
        )
        .fetch_all(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("获取群组成员失败: {:?}", e);
            AppError::InternalServerError("获取群组成员失败".to_string())
        })?;

        let has_more = rows.len() > limit as usize;
        let members = if has_more {
            rows[..limit as usize].to_vec()
        } else {
            rows
        };
        let next_cursor = if has_more && !members.is_empty() {
            Some(members.last().unwrap().id)
        } else {
            None
        };

        Ok(GroupMemberList {
            members,
            has_more,
            next_cursor,
        })
    }

    // 根据位置搜索群组
    pub async fn search_group_by_location(
        &self,
        user_id: i64,
        radius: f64,
        cursor: i64,
        limit: i64,
    ) -> Result<(Vec<GroupRow>, bool, Option<i64>), AppError> {
        let rows = sqlx::query_as!(
            GroupRow,
            r#"
            WITH nearby_groups AS (
                SELECT 
                    g.id,
                    g.group_id,
                    g.name,
                    g.description,
                    g.owner_id,
                    u.nickname as creator_name,
                    g.created_at,
                    g.updated_at as last_active_at,
                    g.latitude,
                    g.longitude,
                    g.location_name,
                    (SELECT COUNT(*) FROM group_members WHERE group_id = g.group_id) as member_count,
                    ST_Distance(
                        ST_MakePoint(g.longitude, g.latitude)::geography,
                        ST_MakePoint(COALESCE(ul.longitude, $3), COALESCE(ul.latitude, $4))::geography
                    ) as distance,
                    (g.password_hash IS NOT NULL) as is_password_required,
                    (EXISTS(SELECT 1 FROM group_members WHERE group_id = g.group_id AND user_id = $1)) as is_member,
                    COALESCE((SELECT role FROM group_members WHERE group_id = g.group_id AND user_id = $1), '') as user_role
                FROM groups g
                JOIN users u ON g.owner_id = u.user_id
                LEFT JOIN user_locations ul ON ul.user_id = $1
                WHERE ($5::bigint = 0 OR g.id < $5::bigint)
            )
            SELECT 
                id,
                group_id::uuid as "group_id!", 
                name as "name!", 
                description, 
                owner_id as "owner_id!", 
                creator_name as "creator_name!", 
                created_at as "created_at!", 
                last_active_at as "last_active_at!", 
                latitude as "latitude!", 
                longitude as "longitude!", 
                location_name as "location_name!", 
                member_count as "member_count!", 
                distance as "distance!", 
                is_password_required as "is_password_required!", 
                is_member as "is_member!", 
                user_role as "user_role!"
            FROM nearby_groups
            WHERE distance <= $2
            ORDER BY id DESC
            LIMIT $6
            "#,
            user_id,
            radius,
            DEFAULT_LONGITUDE,
            DEFAULT_LATITUDE,
            cursor,
            limit + 1
        )
        .fetch_all(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("查询附近群组失败: {:?}", e);
            AppError::InternalServerError("查询附近群组失败".to_string())
        })?;

        let has_more = rows.len() > limit as usize;
        let groups = if has_more {
            rows[..limit as usize].to_vec()
        } else {
            rows
        };
        let next_cursor = if has_more && !groups.is_empty() {
            Some(groups.last().unwrap().id)
        } else {
            None
        };

        Ok((groups, has_more, next_cursor))
    }

    // 根据名称搜索群组
    pub async fn search_group_by_name(
        &self,
        user_id: i64,
        keyword: String,
        cursor: i64,
        limit: i64,
    ) -> Result<(Vec<GroupRow>, bool, Option<i64>), AppError> {
        let rows = sqlx::query_as!(
            GroupRow,
            r#"
            WITH search_groups AS (
                SELECT 
                    g.id,
                    g.group_id,
                    g.name,
                    g.description,
                    g.owner_id,
                    u.nickname as creator_name,
                    g.created_at,
                    g.updated_at as last_active_at,
                    g.latitude,
                    g.longitude,
                    g.location_name,
                    (SELECT COUNT(*) FROM group_members WHERE group_id = g.group_id) as member_count,
                    ST_Distance(
                        ST_MakePoint(g.longitude, g.latitude)::geography,
                        ST_MakePoint(COALESCE(ul.longitude, $4), COALESCE(ul.latitude, $5))::geography
                    ) as distance,
                    (g.password_hash IS NOT NULL) as is_password_required,
                    (EXISTS(SELECT 1 FROM group_members WHERE group_id = g.group_id AND user_id = $1)) as is_member,
                    COALESCE((SELECT role FROM group_members WHERE group_id = g.group_id AND user_id = $1), '') as user_role
                FROM groups g
                JOIN users u ON g.owner_id = u.user_id
                LEFT JOIN user_locations ul ON ul.user_id = $1
                WHERE g.name ILIKE $2 AND ($3::bigint = 0 OR g.id < $3::bigint)
            )
            SELECT 
                id,
                group_id::uuid as "group_id!", 
                name as "name!", 
                description, 
                owner_id as "owner_id!", 
                creator_name as "creator_name!", 
                created_at as "created_at!", 
                last_active_at as "last_active_at!", 
                latitude as "latitude!", 
                longitude as "longitude!", 
                location_name as "location_name!", 
                member_count as "member_count!", 
                distance as "distance!", 
                is_password_required as "is_password_required!", 
                is_member as "is_member!", 
                user_role as "user_role!"
            FROM search_groups
            ORDER BY id DESC
            LIMIT $6
            "#,
            user_id,
            format!("%{}%", keyword),
            cursor,
            DEFAULT_LONGITUDE,
            DEFAULT_LATITUDE,
            limit + 1
        )
        .fetch_all(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("搜索群组失败: {:?}", e);
            AppError::InternalServerError("搜索群组失败".to_string())
        })?;

        let has_more = rows.len() > limit as usize;
        let groups = if has_more {
            rows[..limit as usize].to_vec()
        } else {
            rows
        };
        let next_cursor = if has_more && !groups.is_empty() {
            Some(groups.last().unwrap().id)
        } else {
            None
        };

        Ok((groups, has_more, next_cursor))
    }

    // 根据ID获取群组
    pub async fn search_group_by_id(
        &self,
        user_id: i64,
        group_id: Uuid,
    ) -> Result<GroupRow, AppError> {
        let group = sqlx::query_as!(
            GroupRow,
            r#"
            SELECT 
                g.id,
                g.group_id::uuid as "group_id!",
                g.name as "name!",
                g.description,
                g.owner_id as "owner_id!",
                u.nickname as "creator_name!",
                g.created_at as "created_at!",
                g.updated_at as "last_active_at!",
                g.latitude as "latitude!",
                g.longitude as "longitude!",
                g.location_name as "location_name!",
                (SELECT COUNT(*) FROM group_members WHERE group_id = g.group_id) as "member_count!",
                ST_Distance(
                    ST_MakePoint(g.longitude, g.latitude)::geography,
                    ST_MakePoint(COALESCE(ul.longitude, $3), COALESCE(ul.latitude, $4))::geography
                ) as "distance!",
                (g.password_hash IS NOT NULL) as "is_password_required!",
                (EXISTS(SELECT 1 FROM group_members WHERE group_id = g.group_id AND user_id = $1)) as "is_member!",
                COALESCE((SELECT role FROM group_members WHERE group_id = g.group_id AND user_id = $1), '') as "user_role!"
            FROM groups g
            JOIN users u ON g.owner_id = u.user_id
            LEFT JOIN user_locations ul ON ul.user_id = $1
            WHERE g.group_id = $2
            "#,
            user_id,
            group_id,
            DEFAULT_LONGITUDE,
            DEFAULT_LATITUDE
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            tracing::error!("查询群组失败: {:?}", e);
            AppError::NotFound("群组不存在".to_string())
        })?;

        Ok(group)
    }
}

#[derive(Debug, Clone)]
pub struct SearchGroupByNameRequest {
    pub keyword: String,
    pub limit: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct GroupMembersResponse {
    pub members: Vec<GroupMemberRow>,
    pub has_more: bool,
    pub next_cursor: Option<i64>,
}

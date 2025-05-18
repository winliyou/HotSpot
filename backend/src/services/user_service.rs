use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use serde::Serialize;
use tracing::{debug, error, info, warn};
use uuid;
use sqlx::FromRow;
use std::sync::Arc;

use crate::{
    config::{
        db::Database,
        location::{DEFAULT_LATITUDE, DEFAULT_LOCATION_NAME, DEFAULT_LONGITUDE},
    },
    services::ConfigService,
    utils::{
        jwt::{generate_jwt_token, verify_jwt_token},
        response::AppError,
    },
};

// =============== 返回结构体 ===============

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserRow {
    pub user_id: i64,
    pub nickname: String,
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub distance: f64,
    pub location_name: String,
    pub online_status: String,
}

#[derive(Debug, Serialize)]
pub struct UserLoginResult {
    pub user_id: i64,
    pub nickname: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct AuthUserRow {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug)]
pub struct UserLoginRow {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug, Clone)]
pub struct UserService {
    db: Database,
    config_service: Arc<ConfigService>,
}

impl UserService {
    pub fn new(db: Database, config_service: Arc<ConfigService>) -> Self {
        Self { db, config_service }
    }

    // =============== 验证函数 ===============

    // 检查登录ID是否已存在
    pub async fn is_login_id_taken(&self, login_id: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE login_id = $1)",
            login_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("检查登录ID失败: {:?}", e);
            AppError::InternalServerError("检查登录ID失败".to_string())
        })?;

        Ok(exists.unwrap_or(false))
    }

    // =============== 操作函数 ===============

    // 哈希密码
    pub async fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                error!("密码加密失败: {:?}", e);
                AppError::InternalServerError("密码处理失败".to_string())
            })?
            .to_string();

        Ok(password_hash)
    }

    // 更新用户状态（事务操作）
    pub async fn update_user_presence(
        &self,
        user_id: i64,
        latitude: f64,
        longitude: f64,
        location_name: &str,
    ) -> Result<(), AppError> {
        // 开启事务
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 更新用户最后活跃时间
        sqlx::query!(
            r#"
            UPDATE users 
            SET last_active_at = $1
            WHERE user_id = $2
            "#,
            Utc::now(),
            user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("更新用户状态失败: {:?}", e);
            AppError::InternalServerError("更新用户状态失败".to_string())
        })?;

        // 更新用户位置
        sqlx::query!(
            r#"
            INSERT INTO user_locations (user_id, latitude, longitude, location_name, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id) DO UPDATE
            SET latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude,
                location_name = EXCLUDED.location_name,
                updated_at = EXCLUDED.updated_at
            "#,
            user_id,
            latitude,
            longitude,
            location_name,
            Utc::now()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("更新用户位置失败: {:?}", e);
            AppError::InternalServerError("更新用户位置失败".to_string())
        })?;

        // 提交事务
        tx.commit().await.map_err(|e| {
            error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        Ok(())
    }

    // 仅更新用户位置
    pub async fn update_user_location(
        &self,
        user_id: i64,
        latitude: f64,
        longitude: f64,
        location_name: &str,
    ) -> Result<(), AppError> {
        // 更新用户位置
        sqlx::query!(
            r#"
            INSERT INTO user_locations (user_id, latitude, longitude, location_name, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id) DO UPDATE
            SET latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude,
                location_name = EXCLUDED.location_name,
                updated_at = EXCLUDED.updated_at
            "#,
            user_id,
            latitude,
            longitude,
            location_name,
            Utc::now()
        )
        .execute(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("更新用户位置失败: {:?}", e);
            AppError::InternalServerError("更新用户位置失败".to_string())
        })?;

        Ok(())
    }

    // 用户登录
    pub async fn login(
        &self,
        login_id: &str,
        password_hash: &str,
    ) -> Result<UserLoginResult, AppError> {
        info!("【Service】用户登录: login_id={}", login_id);

        // 检查用户是否存在
        let user = sqlx::query_as!(
            UserLoginRow,
            r#"
            SELECT user_id as "user_id!", nickname as "nickname!"
            FROM users
            WHERE login_id = $1
            "#,
            login_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("用户登录查询失败: {:?}", e);
            AppError::NotFound("用户不存在或密码错误".to_string())
        })?;

        // 验证密码
        let password_hash_from_db = sqlx::query_scalar!(
            r#"
            SELECT password_hash
            FROM users
            WHERE login_id = $1
            "#,
            login_id
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("获取密码哈希失败: {:?}", e);
            AppError::NotFound("用户不存在或密码错误".to_string())
        })?;

        // 确保密码存在
        let password_hash_from_db = match password_hash_from_db {
            Some(hash) => hash,
            None => {
                error!("用户密码未设置: login_id={}", login_id);
                return Err(AppError::NotFound("用户不存在或密码错误".to_string()));
            }
        };

        // 验证密码
        if password_hash != password_hash_from_db {
            warn!("密码验证失败: login_id={}", login_id);
            return Err(AppError::NotFound("用户不存在或密码错误".to_string()));
        }

        // 更新用户最后登录时间
        sqlx::query!(
            r#"
            UPDATE users
            SET last_active_at = $1
            WHERE login_id = $2
            "#,
            Utc::now(),
            login_id
        )
        .execute(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("更新用户最后登录时间失败: {:?}", e);
            AppError::InternalServerError("更新用户状态失败".to_string())
        })?;

        // 生成JWT令牌
        let config = self.config_service.get_config();
        let jwt_secret = &config.jwt_secret;
        let access_expires_in = config.jwt_expires_in.parse::<i64>().unwrap_or(3600);
        let refresh_expires_in = std::env::var("JWT_REFRESH_EXPIRES_IN")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<i64>()
            .unwrap_or(604800);

        // 生成访问令牌
        let (access_token, expires_at) =
            generate_jwt_token(user.user_id, jwt_secret, access_expires_in, false).map_err(|e| {
                error!("生成访问令牌失败: {:?}", e);
                AppError::InternalServerError("生成访问令牌失败".to_string())
            })?;

        let (refresh_token, _) = generate_jwt_token(user.user_id, jwt_secret, refresh_expires_in, false)
            .map_err(|e| {
                error!("生成刷新令牌失败: {:?}", e);
                AppError::InternalServerError("生成刷新令牌失败".to_string())
            })?;

        info!("【Service】用户登录成功: user_id={}", user.user_id);

        Ok(UserLoginResult {
            user_id: user.user_id,
            nickname: user.nickname,
            access_token,
            refresh_token,
            expires_at,
        })
    }

    pub async fn logout(&self, user_id: i64) -> Result<(), AppError> {
        // 更新用户状态
        self.update_user_presence(
            user_id,
            DEFAULT_LATITUDE,
            DEFAULT_LONGITUDE,
            DEFAULT_LOCATION_NAME,
        )
        .await?;

        Ok(())
    }

    // 根据ID搜索用户
    pub async fn get_user_by_id(
        &self,
        user_id: i64,
        current_user_id: i64,
    ) -> Result<UserRow, AppError> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
            SELECT 
                u.user_id,
                u.nickname as "nickname!",
                u.last_active_at as last_active,
                COALESCE(ul.latitude, $3) as "latitude!: f64",
                COALESCE(ul.longitude, $4) as "longitude!: f64",
                COALESCE(ul.location_name, $5) as "location_name!: String",
                COALESCE(
                    ST_Distance(
                        ST_MakePoint(ul.longitude, ul.latitude)::geography,
                        ST_MakePoint(COALESCE(ul2.longitude, $4), COALESCE(ul2.latitude, $3))::geography
                    ),
                    0.0
                ) as "distance!: f64",
                CASE 
                    WHEN u.last_active_at > NOW() - INTERVAL '5 minutes' THEN 'online'
                    ELSE 'offline'
                END as "online_status!: String"
            FROM users u
            LEFT JOIN user_locations ul ON u.user_id = ul.user_id
            LEFT JOIN user_locations ul2 ON ul2.user_id = $1
            WHERE u.user_id = $2
            "#,
            current_user_id,
            user_id,
            DEFAULT_LATITUDE,
            DEFAULT_LONGITUDE,
            DEFAULT_LOCATION_NAME
        )
        .fetch_one(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("查询用户失败: {:?}", e);
            AppError::InternalServerError("查询用户失败".to_string())
        })?;

        Ok(user)
    }

    // 查找附近的用户
    pub async fn find_nearby_users(
        &self,
        longitude: f64,
        latitude: f64,
        radius: f64,
        cursor: i64,
        limit: i64,
        current_user_id: i64,
    ) -> Result<(Vec<UserRow>, bool, Option<i64>), AppError> {
        info!(
            "【Service】开始查找附近用户: 位置=({}, {}), 半径={}, 游标={}, 限制={}",
            longitude, latitude, radius, cursor, limit
        );

        // 直接使用游标参数，不做额外判断，同时使用类型提示
        let user_rows = sqlx::query_as!(
            UserRow,
            r#"
            WITH user_distance AS (
                SELECT
                    u.user_id,
                    u.nickname,
                    u.last_active_at as last_active,
                    coalesce(ul.latitude, $1) as latitude,
                    coalesce(ul.longitude, $2) as longitude,
                    coalesce(ul.location_name, $3) as location_name,
                    COALESCE(
                        ST_Distance(
                            ST_SetSRID(ST_MakePoint(coalesce(ul.longitude, $2), coalesce(ul.latitude, $1)), 4326)::geography,
                            ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography
                        ),
                        0
                    ) as distance,
                    CASE 
                        WHEN u.last_active_at > NOW() - INTERVAL '5 minutes' THEN 'online'
                        ELSE 'offline'
                    END as online_status
                FROM users u
                LEFT JOIN user_locations ul ON u.user_id = ul.user_id
                WHERE 
                    u.user_id != $4 AND
                    u.id < $5 AND
                    COALESCE(
                        ST_Distance(
                            ST_SetSRID(ST_MakePoint(coalesce(ul.longitude, $2), coalesce(ul.latitude, $1)), 4326)::geography,
                            ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography
                        ),
                        0
                    ) <= $6
                ORDER BY u.id DESC
                LIMIT $7 + 1
            )
            SELECT 
                user_id as "user_id!: i64",
                nickname as "nickname!: String",
                last_active,
                latitude as "latitude!: f64",
                longitude as "longitude!: f64",
                location_name as "location_name!: String",
                distance as "distance!: f64",
                online_status as "online_status!: String"
            FROM user_distance
            ORDER BY distance ASC
            "#,
            latitude,
            longitude,
            DEFAULT_LOCATION_NAME,
            current_user_id,
            cursor as i32,
            radius,
            limit as i32
        )
        .fetch_all(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("搜索附近用户失败: {:?}", e);
            AppError::InternalServerError("搜索附近用户失败".to_string())
        })?;

        let has_more = user_rows.len() as i64 > limit;
        let next_cursor_id = if has_more && !user_rows.is_empty() {
            Some(user_rows.last().unwrap().user_id)
        } else {
            None
        };
        let user_rows = if has_more {
            user_rows[..user_rows.len() - 1].to_vec()
        } else {
            user_rows
        };
        info!(
            "【Service】查找附近用户成功: 返回{}个用户，是否有更多={}",
            user_rows.len(),
            has_more
        );
        Ok((user_rows, has_more, next_cursor_id))
    }

    // 根据名称搜索用户
    pub async fn find_users_by_name(
        &self,
        keyword: &str,
        cursor: i64,
        limit: i64,
        current_user_id: i64,
    ) -> Result<(Vec<UserRow>, bool, Option<i64>), AppError> {
        info!(
            "【Service】开始搜索用户: 关键词={}, 游标={}, 限制={}",
            keyword, cursor, limit
        );
        
        // 添加默认位置名称和坐标
        let default_latitude = DEFAULT_LATITUDE;
        let default_longitude = DEFAULT_LONGITUDE;

        // 直接使用游标参数，不做额外判断，同时使用类型提示
        let user_rows = sqlx::query_as!(
            UserRow,
            r#"
            WITH user_search AS (
                SELECT
                    u.user_id,
                    u.nickname,
                    u.last_active_at as last_active,
                    coalesce(ul.latitude, $1) as latitude,
                    coalesce(ul.longitude, $2) as longitude,
                    coalesce(ul.location_name, $3) as location_name,
                    COALESCE(
                        ST_Distance(
                            ST_SetSRID(ST_MakePoint(coalesce(ul.longitude, $2), coalesce(ul.latitude, $1)), 4326)::geography,
                            ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography
                        ),
                        0
                    ) as distance,
                    CASE 
                        WHEN u.last_active_at > NOW() - INTERVAL '5 minutes' THEN 'online'
                        ELSE 'offline'
                    END as online_status
                FROM users u
                LEFT JOIN user_locations ul ON u.user_id = ul.user_id
                WHERE 
                    u.user_id != $4 AND
                    u.id < $5 AND
                    lower(u.nickname) LIKE lower($6)
                ORDER BY u.id DESC
                LIMIT $7 + 1
            )
            SELECT 
                user_id as "user_id!: i64",
                nickname as "nickname!: String",
                last_active,
                latitude as "latitude!: f64",
                longitude as "longitude!: f64",
                location_name as "location_name!: String",
                distance as "distance!: f64",
                online_status as "online_status!: String"
            FROM user_search
            ORDER BY distance ASC
            "#,
            default_latitude,
            default_longitude,
            DEFAULT_LOCATION_NAME,
            current_user_id,
            cursor as i32,
            format!("%{}%", keyword),
            limit as i32
        )
        .fetch_all(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("搜索用户失败: {:?}", e);
            AppError::InternalServerError("搜索用户失败".to_string())
        })?;

        let has_more = user_rows.len() as i64 > limit;
        let next_cursor_id = if has_more && !user_rows.is_empty() {
            Some(user_rows.last().unwrap().user_id)
        } else {
            None
        };
        let user_rows = if has_more {
            user_rows[..user_rows.len() - 1].to_vec()
        } else {
            user_rows
        };
        info!(
            "【Service】搜索用户成功: 返回{}个用户，是否有更多={}",
            user_rows.len(),
            has_more
        );
        Ok((user_rows, has_more, next_cursor_id))
    }

    // 获取认证用户信息（仅返回基本信息，用于认证流程）
    pub async fn get_auth_info(&self, user_id: i64) -> Result<AuthUserRow, AppError> {
        let user = sqlx::query_as!(
            AuthUserRow,
            r#"
            SELECT 
                user_id as "user_id!",
                nickname as "nickname!"
            FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("获取用户认证信息失败: {:?}", e);
            AppError::InternalServerError("获取用户信息失败".to_string())
        })?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        Ok(user)
    }

    // 注册并生成令牌
    pub async fn signup_with_token(
        &self,
        login_id: &str,
        password_hash: &str, // 从前端收到的无盐哈希值
        nickname: &str,
    ) -> Result<UserLoginResult, AppError> {
        info!(
            "【Service】开始用户注册: login_id={}, nickname={}",
            login_id, nickname
        );

        // 检查用户名是否已存在
        if self.is_login_id_taken(login_id).await? {
            warn!("【Service】注册失败: 登录ID已存在 - {}", login_id);
            return Err(AppError::Conflict("该账号已被注册".to_string()));
        }

        debug!("【Service】登录ID检查通过，开始创建用户");

        // 使用Argon2对前端传来的无盐哈希值再次加盐哈希
        // 把前端的哈希值当作"密码"再次哈希
        let argon2_hash = self.hash_password(password_hash).await?;

        // 开启事务
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 创建用户
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (login_id, nickname, password_hash, created_at, updated_at, last_active_at)
            VALUES ($1, $2, $3, $4, $4, $4)
            RETURNING user_id
            "#,
            login_id,
            nickname,
            argon2_hash,  // 存储二次哈希后的密码
            Utc::now()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            error!("创建用户失败: {:?}", e);
            AppError::InternalServerError("创建用户失败".to_string())
        })?;

        debug!("【Service】用户创建成功，用户ID: {}", user_id);

        // 设置初始位置
        sqlx::query!(
            r#"
            INSERT INTO user_locations (user_id, latitude, longitude, location_name, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_id,
            DEFAULT_LATITUDE,
            DEFAULT_LONGITUDE,
            DEFAULT_LOCATION_NAME,
            Utc::now()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("设置用户初始位置失败: {:?}", e);
            AppError::InternalServerError("设置用户位置失败".to_string())
        })?;

        // 提交事务
        tx.commit().await.map_err(|e| {
            error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 生成JWT令牌
        let config = self.config_service.get_config();
        let jwt_secret = &config.jwt_secret;
        let access_expires_in = config.jwt_expires_in.parse::<i64>().unwrap_or(3600);
        // 临时用户使用更长的过期时间（30天）
        let refresh_expires_in = 30 * 24 * 60 * 60; // 30天

        let (access_token, expires_at) =
            generate_jwt_token(user_id, jwt_secret, access_expires_in, true).map_err(|e| {
                error!("生成访问令牌失败: {:?}", e);
                AppError::InternalServerError("生成访问令牌失败".to_string())
            })?;

        let (refresh_token, _) = generate_jwt_token(user_id, jwt_secret, refresh_expires_in, true)
            .map_err(|e| {
                error!("生成刷新令牌失败: {:?}", e);
                AppError::InternalServerError("生成刷新令牌失败".to_string())
            })?;

        info!(
            "【Service】用户注册成功: user_id={}, nickname={}",
            user_id, nickname
        );

        Ok(UserLoginResult {
            user_id,
            nickname: nickname.to_string(),
            access_token,
            refresh_token,
            expires_at,
        })
    }

    // 创建临时用户
    pub async fn create_temp_user(&self, nickname: &str) -> Result<UserLoginResult, AppError> {
        info!("【Service】开始创建临时用户: nickname={}", nickname);

        // 开启事务
        let mut tx = self.db.pg_pool.begin().await.map_err(|e| {
            error!("开启事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 生成临时登录ID
        let temp_login_id = format!("temp_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        debug!("【Service】生成临时登录ID: {}", temp_login_id);

        // 生成临时用户ID - 从99999999999开始
        let temp_user_id = 99999999999 + rand::random::<i64>().abs() % 9000000000;

        // 创建临时用户
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (user_id, login_id, nickname, created_at, updated_at, last_active_at)
            VALUES ($1, $2, $3, $4, $4, $4)
            RETURNING user_id
            "#,
            temp_user_id,
            temp_login_id,
            nickname,
            Utc::now()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            error!("创建临时用户失败: {:?}", e);
            AppError::InternalServerError("创建临时用户失败".to_string())
        })?;

        debug!("【Service】临时用户创建成功，用户ID: {}", user_id);

        // 设置初始位置
        sqlx::query!(
            r#"
            INSERT INTO user_locations (user_id, latitude, longitude, location_name, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_id,
            DEFAULT_LATITUDE,
            DEFAULT_LONGITUDE,
            DEFAULT_LOCATION_NAME,
            Utc::now()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("设置用户初始位置失败: {:?}", e);
            AppError::InternalServerError("设置用户位置失败".to_string())
        })?;

        // 提交事务
        tx.commit().await.map_err(|e| {
            error!("提交事务失败: {:?}", e);
            AppError::InternalServerError("数据库操作失败".to_string())
        })?;

        // 生成JWT令牌
        let config = self.config_service.get_config();
        let jwt_secret = &config.jwt_secret;
        let access_expires_in = config.jwt_expires_in.parse::<i64>().unwrap_or(3600);
        // 临时用户使用更长的过期时间（30天）
        let refresh_expires_in = 30 * 24 * 60 * 60; // 30天

        // 为临时用户生成Token
        let (access_token, expires_at) =
            generate_jwt_token(user_id, jwt_secret, access_expires_in, true).map_err(|e| {
                error!("生成访问令牌失败: {:?}", e);
                AppError::InternalServerError("生成访问令牌失败".to_string())
            })?;

        let (refresh_token, _) = generate_jwt_token(user_id, jwt_secret, refresh_expires_in, true)
            .map_err(|e| {
                error!("生成刷新令牌失败: {:?}", e);
                AppError::InternalServerError("生成刷新令牌失败".to_string())
            })?;

        info!(
            "【Service】临时用户创建成功: user_id={}, nickname={}",
            user_id, nickname
        );

        Ok(UserLoginResult {
            user_id,
            nickname: nickname.to_string(),
            access_token,
            refresh_token,
            expires_at,
        })
    }

    // 判断用户ID是否为临时用户
    pub fn is_temp_user(&self, user_id: i64) -> bool {
        // 临时用户ID从99999999999开始
        user_id >= 99999999999
    }

    // 刷新令牌
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<UserLoginResult, AppError> {
        // 验证刷新令牌
        let config = self.config_service.get_config();
        let jwt_secret = &config.jwt_secret;
        let claims = verify_jwt_token(refresh_token, jwt_secret)
            .map_err(|_| AppError::Unauthorized("刷新令牌无效或已过期".to_string()))?;

        // 获取用户信息
        let user_id = claims.sub;
        // 使用claims中的临时用户标志
        let is_temp = claims.temp;
        
        // 从users表中获取用户信息
        let nickname = sqlx::query_scalar!(
            r#"
            SELECT nickname
            FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db.pg_pool)
        .await
        .map_err(|e| {
            error!("获取用户信息失败: {:?}", e);
            AppError::NotFound("用户不存在".to_string())
        })?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 生成新的访问令牌
        let access_expires_in = config.jwt_expires_in.parse::<i64>().unwrap_or(3600);
        // 根据用户类型设置刷新令牌的过期时间
        let refresh_expires_in = if is_temp {
            30 * 24 * 60 * 60 // 临时用户30天
        } else {
            std::env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "604800".to_string()) // 正式用户默认7天
                .parse::<i64>()
                .unwrap_or(604800)
        };

        let (new_access_token, expires_at) =
            generate_jwt_token(user_id, &jwt_secret, access_expires_in, false).map_err(|e| AppError::InternalServerError(format!("生成访问令牌失败: {:?}", e)))?;

        let (new_refresh_token, _) =
            generate_jwt_token(user_id, &jwt_secret, refresh_expires_in, false).map_err(|e| AppError::InternalServerError(format!("生成刷新令牌失败: {:?}", e)))?;

        Ok(UserLoginResult {
            user_id,
            nickname,
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_at,
        })
    }
}

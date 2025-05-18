use axum::{
    Json, debug_handler,
    extract::{Extension, State},
};
use std::sync::Arc;
use tracing::{debug, info};

use crate::{
    middleware::auth::AppState,
    models::{
        api::user::AuthUser,
        api::user::{
            AuthResponse, CreateTempUserRequest, LoginRequest, RefreshTokenRequest,
            RefreshTokenResponse, RegisterRequest, SearchUserByIdRequest, SearchUserByIdResponse,
            SearchUserByLocationRequest, SearchUserByLocationResponse, SearchUserByNameRequest,
            SearchUserByNameResponse, UpdateLocationRequest, UserInfo,
        },
    },
    utils::response::{ApiResponse, AppError, PaginatedResponse, PaginationMeta, success_response},
    validators::{common_validator, user_validator},
};

// 用户注册
#[debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    info!(
        "【Controller】开始用户注册处理: login_id={}",
        payload.login_id
    );
    debug!(
        "【Controller】注册请求参数: login_id={}, nickname={:?}",
        payload.login_id, payload.nickname
    );

    // 验证请求参数
    user_validator::validate_user_registration(
        &payload.login_id,
        Some(&payload.password),
        payload.nickname.as_deref(),
    )?;

    // 直接调用服务层的注册并生成令牌方法
    let user_login_result = state
        .user_service
        .signup_with_token(
            &payload.login_id,
            &payload.password,
            &payload.nickname.unwrap_or_else(|| "用户".to_string()),
        )
        .await?;

    // 组装响应
    let response = AuthResponse {
        user_id: user_login_result.user_id,
        nickname: user_login_result.nickname,
        access_token: user_login_result.access_token,
        refresh_token: user_login_result.refresh_token,
        expires_at: user_login_result.expires_at,
    };

    info!(
        "【Controller】用户注册成功: user_id={}",
        user_login_result.user_id
    );
    Ok(success_response(response))
}

// 用户登录
#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    info!(
        "【Controller】开始用户登录处理: login_id={}",
        payload.login_id
    );

    // 1. 调用登录接口
    let user_login_result = state
        .user_service
        .login(&payload.login_id, &payload.password)
        .await?;

    // 2. 组装响应
    let response = AuthResponse {
        user_id: user_login_result.user_id,
        nickname: user_login_result.nickname,
        access_token: user_login_result.access_token,
        refresh_token: user_login_result.refresh_token,
        expires_at: user_login_result.expires_at,
    };

    info!(
        "【Controller】用户登录成功: user_id={}",
        user_login_result.user_id
    );
    Ok(success_response(response))
}

// 创建临时用户
#[debug_handler]
pub async fn create_temp_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTempUserRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    info!(
        "【Controller】开始创建临时用户处理: nickname={:?}",
        payload.nickname
    );

    // 验证昵称（如果提供）
    if let Some(nickname) = &payload.nickname {
        user_validator::validate_nickname(nickname)?;
    }

    // 调用service层实现临时用户创建
    let user_login_result = state
        .user_service
        .create_temp_user(&payload.nickname.unwrap_or_else(|| "临时用户".to_string()))
        .await?;

    // 构建响应
    let response = AuthResponse {
        user_id: user_login_result.user_id,
        nickname: user_login_result.nickname,
        access_token: user_login_result.access_token,
        refresh_token: user_login_result.refresh_token,
        expires_at: user_login_result.expires_at,
    };

    info!(
        "【Controller】临时用户创建成功: user_id={}",
        user_login_result.user_id
    );
    Ok(success_response(response))
}

// 根据位置搜索用户
#[debug_handler]
pub async fn search_user_by_location(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchUserByLocationRequest>,
) -> Result<Json<ApiResponse<SearchUserByLocationResponse>>, AppError> {
    info!(
        "【Controller】开始根据位置搜索用户: 用户ID={}, 经度={}, 纬度={}, 半径={}",
        current_user.user_id, payload.longitude, payload.latitude, payload.radius
    );

    // 参数验证
    common_validator::validate_location_params(payload.latitude, payload.longitude)?;
    common_validator::validate_search_radius(payload.radius, 10000.0)?; // 最大半径10公里

    // 参数转换
    let longitude = payload.longitude;
    let latitude = payload.latitude;
    let radius = payload.radius;
    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(10);

    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?; // 最大每页50条
    }

    // 调用service层方法
    let (users, has_more, next_cursor) = state
        .user_service
        .find_nearby_users(
            longitude,
            latitude,
            radius,
            cursor,
            limit,
            current_user.user_id,
        )
        .await?;

    // 转换为API响应模型
    let user_infos: Vec<UserInfo> = users
        .into_iter()
        .map(|user| UserInfo {
            user_id: user.user_id,
            nickname: user.nickname,
            last_active: user.last_active,
            latitude: user.latitude,
            longitude: user.longitude,
            distance: user.distance,
            location_name: user.location_name,
            online_status: user.online_status,
        })
        .collect();

    info!(
        "【Controller】根据位置搜索用户成功: 找到{}个用户",
        user_infos.len()
    );
    // 构建分页响应
    let response = PaginatedResponse {
        items: user_infos,
        pagination: PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    Ok(success_response(response))
}

// 根据名称搜索用户
#[debug_handler]
pub async fn search_user_by_name(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchUserByNameRequest>,
) -> Result<Json<ApiResponse<SearchUserByNameResponse>>, AppError> {
    info!(
        "【Controller】开始根据名称搜索用户: 当前用户ID={}, 搜索关键词={}",
        current_user.user_id, payload.keyword
    );

    // 验证关键字
    common_validator::validate_string_length(&payload.keyword, "搜索关键词", 1, 50)?;

    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?; // 最大每页50条
    }

    // 参数转换
    let keyword = &payload.keyword;
    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(10);

    // 调用service层方法
    let (users, has_more, next_cursor) = state
        .user_service
        .find_users_by_name(keyword, cursor, limit, current_user.user_id)
        .await?;

    // 转换为API响应模型
    let user_infos: Vec<UserInfo> = users
        .into_iter()
        .map(|user| UserInfo {
            user_id: user.user_id,
            nickname: user.nickname,
            last_active: user.last_active,
            latitude: user.latitude,
            longitude: user.longitude,
            distance: user.distance,
            location_name: user.location_name,
            online_status: user.online_status,
        })
        .collect();

    info!(
        "【Controller】根据名称搜索用户成功: 关键词={}, 找到{}个用户",
        payload.keyword,
        user_infos.len()
    );

    // 构建分页响应
    let response = PaginatedResponse {
        items: user_infos,
        pagination: PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    Ok(success_response(response))
}

// 根据ID搜索用户
#[debug_handler]
pub async fn search_user_by_id(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchUserByIdRequest>,
) -> Result<Json<ApiResponse<SearchUserByIdResponse>>, AppError> {
    info!(
        "【Controller】开始根据ID搜索用户: 当前用户ID={}, 目标用户ID={}",
        current_user.user_id, payload.user_id
    );

    // 参数转换
    let user_id = payload.user_id;

    // 验证要查找的用户ID
    if user_id <= 0 {
        return Err(AppError::BadRequest("无效的用户ID".to_string()));
    }

    // 调用service层方法
    let user = state
        .user_service
        .get_user_by_id(user_id, current_user.user_id)
        .await?;

    info!(
        "【Controller】根据ID搜索用户成功: 用户ID={}, 昵称={}",
        user.user_id, user.nickname
    );

    // 转换为API响应模型
    let user_info = UserInfo {
        user_id: user.user_id,
        nickname: user.nickname,
        last_active: user.last_active,
        latitude: user.latitude,
        longitude: user.longitude,
        distance: user.distance,
        location_name: user.location_name,
        online_status: user.online_status,
    };

    // 直接返回UserInfo作为SearchUserByIdResponse
    Ok(success_response(user_info))
}

// 更新用户位置
#[debug_handler]
pub async fn update_location(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateLocationRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    info!(
        "【Controller】开始更新用户位置: 用户ID={}, 经度={}, 纬度={}, 位置名称={:?}",
        current_user.user_id, payload.longitude, payload.latitude, payload.location_name
    );

    // 验证位置参数
    common_validator::validate_location_params(payload.latitude, payload.longitude)?;

    common_validator::validate_location_name(&payload.location_name)?;

    // 调用service层方法
    state
        .user_service
        .update_user_location(
            current_user.user_id,
            payload.latitude,
            payload.longitude,
            &payload.location_name,
        )
        .await?;

    info!(
        "【Controller】用户位置更新成功: 用户ID={}",
        current_user.user_id
    );
    Ok(success_response(()))
}

// 刷新令牌
#[debug_handler]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, AppError> {
    info!("【Controller】开始刷新令牌处理");
    debug!(
        "【Controller】刷新令牌请求: refresh_token={}",
        payload.refresh_token
    );

    // 调用service层方法
    let token_result = state
        .user_service
        .refresh_token(&payload.refresh_token)
        .await?;

    // 构建响应
    let response = RefreshTokenResponse {
        access_token: token_result.access_token,
        refresh_token: token_result.refresh_token,
        expires_at: token_result.expires_at,
    };

    info!(
        "【Controller】令牌刷新成功: 过期时间={}",
        token_result.expires_at
    );
    Ok(success_response(response))
}

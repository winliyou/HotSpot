use std::sync::Arc;

use axum::{Extension, Json, debug_handler, extract::State};
use tracing::{debug, info};

use crate::{
    middleware::auth::AppState,
    models::api::checkin::{
        CheckInInfo, CreateCheckinRequest, CreateCheckinResponse, DeleteCheckinRequest,
        DeleteCheckinResponse, SearchCheckInByIdRequest, SearchCheckinByLocationResponse,
        SearchCheckinsByLocationRequest, SearchCheckinsByTagsRequest, SearchCheckinsByTagsResponse,
        UserCheckinsHistoryRequest, UserCheckinsHistoryResponse,
    },
    models::api::user::AuthUser,
    utils::response::{ApiResponse, AppError, PaginatedResponse, PaginationMeta, success_response},
    validators::{checkin_validator, common_validator},
};

// 创建签到
#[debug_handler]
pub async fn create_checkin(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCheckinRequest>,
) -> Result<Json<ApiResponse<CreateCheckinResponse>>, AppError> {
    info!(
        "【Controller】开始创建签到: 用户ID={}, 位置=({}, {}), 位置名称={}",
        current_user.user_id, payload.latitude, payload.longitude, payload.location_name
    );
    debug!(
        "【Controller】签到内容: {}, 标签: {:?}",
        &payload.description,
        payload.tags.as_deref().unwrap_or(&[])
    );

    // 验证签到参数
    checkin_validator::validate_checkin_creation_request(
        &payload.description,
        payload.latitude,
        payload.longitude,
        &payload.location_name,
        payload.tags.as_deref().unwrap_or(&[]),
    )?;

    let checkin_id = state
        .checkin_service
        .create_checkin(
            current_user.user_id,
            &payload.description,
            payload.latitude,
            payload.longitude,
            &payload.location_name,
            payload.tags.as_deref().unwrap_or(&[]),
        )
        .await?;

    // 通过WebSocket发送签到通知
    let preview = payload.description.chars().take(50).collect::<String>();
    let _ = state
        .ws_service
        .send_checkin_notification(
            &checkin_id,
            current_user.user_id,
            &current_user.nickname,
            payload.latitude,
            payload.longitude,
            &payload.location_name,
            Some(&preview),
        )
        .await;

    info!(
        "【Controller】签到创建成功: 用户ID={}, 签到ID={}",
        current_user.user_id, checkin_id
    );
    Ok(success_response(CreateCheckinResponse { checkin_id }))
}

// 删除签到
#[debug_handler]
pub async fn delete_checkin(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteCheckinRequest>,
) -> Result<Json<ApiResponse<DeleteCheckinResponse>>, AppError> {
    info!(
        "【Controller】开始删除签到: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );

    let checkin_id = &payload.checkin_id;
    state
        .checkin_service
        .delete_checkin(current_user.user_id, checkin_id)
        .await?;

    info!(
        "【Controller】签到删除成功: 用户ID={}, 签到ID={}",
        current_user.user_id, checkin_id
    );
    Ok(success_response(DeleteCheckinResponse { success: true }))
}

// 获取用户签到历史
#[debug_handler]
pub async fn history(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCheckinsHistoryRequest>,
) -> Result<Json<ApiResponse<UserCheckinsHistoryResponse>>, AppError> {
    let user_id = payload.user_id.unwrap_or_else(|| current_user.user_id);
    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(10);

    info!(
        "【Controller】开始获取用户签到历史: 当前用户ID={}, 查询用户ID={}, 游标={}, 限制={}",
        current_user.user_id, user_id, cursor, limit
    );

    // 验证分页参数
    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?;
    }

    let db_items = state
        .checkin_service
        .search_checkins_by_user(user_id, cursor, limit)
        .await?;

    debug!("【Controller】获取到{}条用户签到记录", db_items.len());

    let mut items = Vec::with_capacity(db_items.len());
    for db_info in db_items.iter() {
        items.push(CheckInInfo {
            id: db_info.id,
            checkin_id: db_info.checkin_id.to_string(),
            user_id: db_info.user_id,
            nickname: db_info.nickname.clone(),
            description: db_info.description.clone(),
            latitude: db_info.latitude,
            longitude: db_info.longitude,
            location_name: db_info.location_name.clone(),
            distance: db_info.distance,
            created_at: db_info.created_at,
            tags: db_info.tags.clone(),
            likes_count: db_info.likes_count,
            comments_count: 0,
            liked_by_me: db_info.liked_by_me,
        });
    }
    let has_more = items.len() as i64 > payload.pagination.limit.unwrap_or(10);
    let next_cursor = items.last().map(|item| item.id);
    let items = if has_more {
        items[..items.len() - 1].to_vec()
    } else {
        items
    };
    let response = PaginatedResponse {
        items: items.clone(),
        pagination: PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    info!(
        "【Controller】获取用户签到历史成功: 用户ID={}, 返回{}条记录, 是否有更多={}",
        user_id,
        items.len(),
        has_more
    );
    Ok(success_response(response))
}

// 获取附近签到
#[debug_handler]
pub async fn search_checkin_by_location(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchCheckinsByLocationRequest>,
) -> Result<Json<ApiResponse<SearchCheckinByLocationResponse>>, AppError> {
    info!(
        "【Controller】开始获取附近签到: 用户ID={}, 位置=({}, {}), 半径={}",
        current_user.user_id, payload.latitude, payload.longitude, payload.radius
    );

    // 验证位置参数
    common_validator::validate_location_params(payload.latitude, payload.longitude)?;

    // 验证搜索半径
    checkin_validator::validate_search_radius(payload.radius)?;

    // 验证分页参数
    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(10);

    if let (Some(c), Some(l)) = (payload.pagination.cursor, payload.pagination.limit) {
        common_validator::validate_pagination(c, l, 50)?;
    }

    let db_items = state
        .checkin_service
        .search_checkins_by_location(
            current_user.user_id,
            payload.latitude,
            payload.longitude,
            payload.radius,
            cursor,
            limit,
        )
        .await?;

    debug!("【Controller】获取到{}条附近签到记录", db_items.len());

    let mut items = Vec::with_capacity(db_items.len());
    for db_info in db_items.iter() {
        items.push(CheckInInfo {
            id: db_info.id,
            checkin_id: db_info.checkin_id.clone(),
            user_id: db_info.user_id,
            nickname: db_info.nickname.clone(),
            description: db_info.description.clone(),
            latitude: db_info.latitude,
            longitude: db_info.longitude,
            location_name: db_info.location_name.clone(),
            distance: db_info.distance,
            created_at: db_info.created_at,
            tags: db_info.tags.clone(),
            likes_count: db_info.likes_count,
            comments_count: db_info.comments_count,
            liked_by_me: db_info.liked_by_me,
        });
    }
    let has_more = items.len() as i64 > limit;
    let next_cursor = items.last().map(|item| item.id);
    let items = if has_more {
        items[..items.len() - 1].to_vec()
    } else {
        items
    };
    let response = PaginatedResponse {
        items: items.clone(),
        pagination: PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    info!(
        "【Controller】获取附近签到成功: 用户ID={}, 位置=({}, {}), 返回{}条记录, 是否有更多={}",
        current_user.user_id,
        payload.latitude,
        payload.longitude,
        items.len(),
        has_more
    );
    Ok(success_response(response))
}

// 获取签到详情
#[debug_handler]
pub async fn search_checkin_by_id(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchCheckInByIdRequest>,
) -> Result<Json<ApiResponse<CheckInInfo>>, AppError> {
    info!(
        "【Controller】开始获取签到详情: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );

    let db_info = state
        .checkin_service
        .search_checkin_by_id(current_user.user_id, &payload.checkin_id)
        .await?;
    let api_info = CheckInInfo {
        id: db_info.id,
        checkin_id: db_info.checkin_id.clone(),
        user_id: db_info.user_id,
        nickname: db_info.nickname,
        description: db_info.description,
        latitude: db_info.latitude,
        longitude: db_info.longitude,
        location_name: db_info.location_name,
        distance: db_info.distance,
        created_at: db_info.created_at,
        tags: db_info.tags,
        likes_count: db_info.likes_count,
        comments_count: db_info.comments_count,
        liked_by_me: db_info.liked_by_me,
    };

    info!(
        "【Controller】获取签到详情成功: 用户ID={}, 签到ID={}, 发布用户ID={}",
        current_user.user_id, payload.checkin_id, db_info.user_id
    );
    Ok(success_response(api_info))
}

// 根据标签搜索签到
#[debug_handler]
pub async fn search_checkin_by_tags(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchCheckinsByTagsRequest>,
) -> Result<Json<ApiResponse<SearchCheckinsByTagsResponse>>, AppError> {
    info!(
        "【Controller】开始根据标签搜索签到: 用户ID={}, 标签={:?}",
        current_user.user_id, payload.tags
    );

    let cursor = payload.pagination.cursor.unwrap_or(0);
    let limit = payload.pagination.limit.unwrap_or(10);

    let db_items = state
        .checkin_service
        .search_checkins_by_tags(current_user.user_id, &payload.tags, cursor, limit)
        .await?;

    debug!("【Controller】根据标签搜索获取到{}条记录", db_items.len());

    let mut items = Vec::with_capacity(db_items.len());
    for db_info in db_items.iter() {
        items.push(CheckInInfo {
            id: db_info.id,
            checkin_id: db_info.checkin_id.to_string(),
            user_id: db_info.user_id,
            nickname: db_info.nickname.clone(),
            description: db_info.description.clone(),
            latitude: db_info.latitude,
            longitude: db_info.longitude,
            location_name: db_info.location_name.clone(),
            distance: db_info.distance,
            created_at: db_info.created_at,
            tags: db_info.tags.clone(),
            likes_count: db_info.likes_count,
            comments_count: db_info.comments_count,
            liked_by_me: db_info.liked_by_me,
        });
    }
    let has_more = items.len() as i64 > limit;
    let next_cursor = items.last().map(|item| item.id);
    let items = if has_more {
        items[..items.len() - 1].to_vec()
    } else {
        items
    };
    let response = PaginatedResponse {
        items: items.clone(),
        pagination: PaginationMeta {
            has_more,
            next_cursor,
        },
    };

    info!(
        "【Controller】根据标签搜索签到成功: 用户ID={}, 标签={:?}, 返回{}条记录, 是否有更多={}",
        current_user.user_id,
        payload.tags,
        items.len(),
        has_more
    );
    Ok(success_response(response))
}

// 点赞签到
#[debug_handler]
pub async fn like_checkin(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchCheckInByIdRequest>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    info!(
        "【Controller】开始点赞签到: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );

    state
        .checkin_service
        .like_checkin(current_user.user_id, &payload.checkin_id)
        .await?;

    info!(
        "【Controller】点赞签到成功: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );
    Ok(success_response(true))
}

// 取消点赞
#[debug_handler]
pub async fn unlike_checkin(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchCheckInByIdRequest>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    info!(
        "【Controller】开始取消点赞: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );

    state
        .checkin_service
        .unlike_checkin(current_user.user_id, &payload.checkin_id)
        .await?;

    info!(
        "【Controller】取消点赞成功: 用户ID={}, 签到ID={}",
        current_user.user_id, payload.checkin_id
    );
    Ok(success_response(true))
}

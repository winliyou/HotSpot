use std::sync::Arc;

use axum::{Extension, Json, debug_handler, extract::State};

use crate::{
    middleware::auth::AppState,
    models::api::{group::*, user::AuthUser},
    utils::response::{ApiResponse, AppError, success_response},
    models::constants::group_roles,
    validators::{group_validator, common_validator},
};

// 创建群组
#[debug_handler]
pub async fn create_group(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<Json<ApiResponse<CreateGroupResponse>>, AppError> {
    // 验证请求参数
    group_validator::validate_group_creation_request(payload.description.as_ref())?;
    
    // 验证名称长度
    group_validator::validate_group_name(&payload.name)?;
    
    // 验证位置名称
    common_validator::validate_location_name(&payload.location_name)?;
    
    // 验证密码长度
    group_validator::validate_group_password(payload.password.as_deref())?;
    
    let group_id = state
        .group_service
        .create_group(
            payload.name.as_str(),
            payload.description.as_deref(),
            current_user.user_id,
            payload.latitude,
            payload.longitude,
            payload.location_name.as_str(),
            payload.password.as_deref(),
        )
        .await?;
    Ok(success_response(CreateGroupResponse { group_id }))
}

// 获取附近群组
#[debug_handler]
pub async fn search_group_by_location(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchGroupByLocationRequest>,
) -> Result<Json<ApiResponse<SearchGroupByLocationResponse>>, AppError> {
    // 验证请求参数
    group_validator::validate_search_radius(payload.radius)?;
    common_validator::validate_location_params(payload.latitude, payload.longitude)?;
    
    // 验证分页参数
    let cursor = payload.cursor.unwrap_or(0);
    let limit = payload.limit.unwrap_or(10);
    
    if let (Some(c), Some(l)) = (payload.cursor, payload.limit) {
        common_validator::validate_pagination(c, l, 50)?;
    }
    
    let (groups, has_more, next_cursor) = state
        .group_service
        .search_group_by_location(current_user.user_id, payload.radius, cursor, limit)
        .await?;

    let items = groups
        .into_iter()
        .map(|group| {
            // 验证角色
            if !group.user_role.is_empty() && !group_roles::is_valid_role(&group.user_role) {
                tracing::warn!("搜索群组返回了无效的用户角色: {}", group.user_role);
            }
            
            GroupInfo {
                group_id: group.group_id,
                name: group.name,
                description: group.description,
                owner_id: group.owner_id,
                creator_name: group.creator_name,
                created_at: group.created_at,
                last_active_at: group.last_active_at,
                latitude: group.latitude,
                longitude: group.longitude,
                location_name: group.location_name,
                member_count: group.member_count,
                distance: group.distance,
                is_password_required: group.is_password_required,
                is_member: group.is_member,
                user_role: group.user_role,
            }
        })
        .collect();

    let response = SearchGroupByLocationResponse {
        groups: items,
        has_more,
        next_cursor,
    };

    Ok(success_response(response))
}

// 按照名称搜索群组
#[debug_handler]
pub async fn search_group_by_name(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchGroupByNameRequest>,
) -> Result<Json<ApiResponse<SearchGroupByNameResponse>>, AppError> {
    // 验证关键字
    common_validator::validate_string_length(&payload.keyword, "搜索关键词", 1, 50)?;
    
    // 验证分页参数
    let cursor = payload.cursor.unwrap_or(0);
    let limit = payload.limit.unwrap_or(10);
    
    if let (Some(c), Some(l)) = (payload.cursor, payload.limit) {
        common_validator::validate_pagination(c, l, 50)?;
    }
    
    let (groups, has_more, next_cursor) = state
        .group_service
        .search_group_by_name(current_user.user_id, payload.keyword.clone(), cursor, limit)
        .await?;

    let items = groups
        .into_iter()
        .map(|group| {
            // 验证角色
            if !group.user_role.is_empty() && !group_roles::is_valid_role(&group.user_role) {
                tracing::warn!("搜索群组返回了无效的用户角色: {}", group.user_role);
            }
            
            GroupInfo {
                group_id: group.group_id,
                name: group.name,
                description: group.description,
                owner_id: group.owner_id,
                creator_name: group.creator_name,
                created_at: group.created_at,
                last_active_at: group.last_active_at,
                latitude: group.latitude,
                longitude: group.longitude,
                location_name: group.location_name,
                member_count: group.member_count,
                distance: group.distance,
                is_password_required: group.is_password_required,
                is_member: group.is_member,
                user_role: group.user_role,
            }
        })
        .collect();

    let response = SearchGroupByNameResponse {
        groups: items,
        has_more,
        next_cursor,
    };

    Ok(success_response(response))
}

// 按照group id搜索群组
#[debug_handler]
pub async fn search_group_by_id(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchGroupByIdRequest>,
) -> Result<Json<ApiResponse<SearchGroupByIdResponse>>, AppError> {
    let result = state
        .group_service
        .search_group_by_id(current_user.user_id, payload.group_id)
        .await?;
        
    // 验证用户角色是否合法
    if !result.user_role.is_empty() && !group_roles::is_valid_role(&result.user_role) {
        tracing::warn!("搜索群组返回了无效的用户角色: {}", result.user_role);
    }
    
    let info = GroupInfo {
        group_id: result.group_id,
        name: result.name,
        description: result.description,
        owner_id: result.owner_id,
        creator_name: result.creator_name,
        created_at: result.created_at,
        last_active_at: result.last_active_at,
        latitude: result.latitude,
        longitude: result.longitude,
        member_count: result.member_count,
        distance: result.distance,
        location_name: result.location_name,
        is_password_required: result.is_password_required,
        is_member: result.is_member,
        user_role: result.user_role,
    };
    Ok(success_response(info))
}

// 加入群组
#[debug_handler]
pub async fn join_group(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JoinGroupRequest>,
) -> Result<Json<ApiResponse<JoinGroupResponse>>, AppError> {
    // 检查群组是否存在
    if !state
        .group_service
        .check_group_exists(payload.group_id)
        .await?
    {
        return Err(AppError::NotFound("群组不存在".to_string()));
    }

    // 检查用户是否已经在群组中
    if state
        .group_service
        .check_user_in_group(payload.group_id, current_user.user_id)
        .await?
    {
        return Err(AppError::BadRequest("您已经是群组成员".to_string()));
    }

    // 检查群组是否需要密码
    let need_password = state
        .group_service
        .check_group_need_password(payload.group_id)
        .await?;
    if need_password {
        match payload.password {
            Some(ref password) => {
                if !state
                    .group_service
                    .verify_group_password(payload.group_id, password)
                    .await?
                {
                    return Err(AppError::BadRequest("密码错误".to_string()));
                }
            }
            None => return Err(AppError::BadRequest("该群组需要密码".to_string())),
        }
    }

    let result = state
        .group_service
        .join_group(payload.group_id, current_user.user_id)
        .await?;

    let role = result.role.unwrap_or_default();
    // 验证返回的角色是否有效
    group_validator::validate_group_role(&role)?;

    let response = crate::models::api::group::JoinGroupResponse {
        success: result.success,
        role,
    };
    Ok(success_response(response))
}

// 离开群组
#[debug_handler]
pub async fn leave_group(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LeaveGroupRequest>,
) -> Result<Json<ApiResponse<LeaveGroupResponse>>, AppError> {
    // 检查群组是否存在
    if !state
        .group_service
        .check_group_exists(payload.group_id)
        .await?
    {
        return Err(AppError::NotFound("群组不存在".to_string()));
    }

    // 检查用户是否在群组中
    if !state
        .group_service
        .check_user_in_group(payload.group_id, current_user.user_id)
        .await?
    {
        return Err(AppError::BadRequest("您不是群组成员".to_string()));
    }

    // 检查用户是否是群主
    if state
        .group_service
        .check_user_is_group_owner(payload.group_id, current_user.user_id)
        .await?
    {
        return Err(AppError::BadRequest(
            "群主不能直接离开群组，请先转让群主身份".to_string(),
        ));
    }

    let result = state
        .group_service
        .leave_group(payload.group_id, current_user.user_id)
        .await?;
    let response = LeaveGroupResponse {
        success: result.success,
    };
    Ok(success_response(response))
}

// 获取群组成员
#[debug_handler]
pub async fn group_members(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GroupMembersRequest>,
) -> Result<Json<ApiResponse<GroupMembersResponse>>, AppError> {
    // 验证请求参数
    let cursor = payload.cursor.unwrap_or(0);
    let limit = payload.limit.unwrap_or(20);
    if payload.cursor.is_some() && payload.limit.is_some() {
        common_validator::validate_pagination(payload.cursor.unwrap(), payload.limit.unwrap(), 100)?;
    }
    
    // 验证群组ID
    if !state.group_service.check_group_exists(payload.group_id).await? {
        return Err(AppError::NotFound("群组不存在".to_string()));
    }

    // 验证用户是否是群组成员
    let is_member = state
        .group_service
        .check_user_in_group(payload.group_id, current_user.user_id)
        .await?;
    if !is_member {
        return Err(AppError::NotFound("您不是该群组成员".to_string()));
    }

    let results = state
        .group_service
        .get_group_members(payload.group_id, cursor, limit, current_user.user_id)
        .await?;

    // 转换为API响应格式
    let response = GroupMembersResponse {
        members: results.members.into_iter().map(|m| GroupMemberInfo {
            user_id: m.user_id,
            nickname: m.nickname.unwrap_or_else(|| "未知用户".to_string()),
            last_active: m.last_active.unwrap_or_else(|| "从未活跃".to_string()),
            role: m.role.unwrap_or_else(|| "member".to_string()),
            join_time: m.join_time.unwrap_or_else(chrono::Utc::now),
        }).collect(),
        has_more: results.has_more,
        next_cursor: results.next_cursor,
    };

    Ok(success_response(response))
}

// 踢出成员
#[debug_handler]
pub async fn kick_member(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<KickMemberRequest>,
) -> Result<Json<ApiResponse<KickMemberResponse>>, AppError> {
    // 检查群组是否存在
    if !state
        .group_service
        .check_group_exists(payload.group_id)
        .await?
    {
        return Err(AppError::NotFound("群组不存在".to_string()));
    }

    // 检查被踢用户是否在群组中
    if !state
        .group_service
        .check_user_in_group(payload.group_id, payload.user_id)
        .await?
    {
        return Err(AppError::BadRequest("该用户不是群组成员".to_string()));
    }

    // 检查操作者权限
    let is_owner = state
        .group_service
        .check_user_is_group_owner(payload.group_id, current_user.user_id)
        .await?;
    let is_admin = state
        .group_service
        .check_user_is_group_admin(payload.group_id, current_user.user_id)
        .await?;
    if !is_owner && !is_admin {
        return Err(AppError::BadRequest("您没有权限执行此操作".to_string()));
    }

    // 检查被踢用户是否是群主
    if state
        .group_service
        .check_user_is_group_owner(payload.group_id, payload.user_id)
        .await?
    {
        return Err(AppError::BadRequest("不能踢出群主".to_string()));
    }

    // 如果操作者是管理员，检查被踢用户是否也是管理员
    if !is_owner && is_admin {
        if state
            .group_service
            .check_user_is_group_admin(payload.group_id, payload.user_id)
            .await?
        {
            return Err(AppError::BadRequest("管理员不能踢出其他管理员".to_string()));
        }
    }

    let result = state
        .group_service
        .kick_member(payload.group_id, payload.user_id, current_user.user_id)
        .await?;
    let response = KickMemberResponse {
        success: result.success,
    };
    Ok(success_response(response))
}

// 转让群主
pub async fn transfer_ownership(
    Extension(current_user): Extension<AuthUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TransferOwnershipRequest>,
) -> Result<Json<ApiResponse<TransferOwnershipResponse>>, AppError> {
    // 检查群组是否存在
    if !state
        .group_service
        .check_group_exists(payload.group_id)
        .await?
    {
        return Err(AppError::NotFound("群组不存在".to_string()));
    }

    // 检查当前用户是否是群主
    if !state
        .group_service
        .check_user_is_group_owner(payload.group_id, current_user.user_id)
        .await?
    {
        return Err(AppError::BadRequest("只有群主才能转让群主身份".to_string()));
    }

    // 检查新群主是否在群组中
    if !state
        .group_service
        .check_user_in_group(payload.group_id, payload.new_owner_id)
        .await?
    {
        return Err(AppError::BadRequest("新群主必须是群组成员".to_string()));
    }
    
    // 验证目标角色是否合法
    group_validator::validate_group_role(group_roles::OWNER)?;
    group_validator::validate_group_role(group_roles::MEMBER)?;

    let result = state
        .group_service
        .transfer_ownership(payload.group_id, payload.new_owner_id, current_user.user_id)
        .await?;
    let response = TransferOwnershipResponse {
        success: result.success,
    };
    Ok(success_response(response))
}

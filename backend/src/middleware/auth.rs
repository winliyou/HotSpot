use crate::{
    models::api::user::AuthUser,
    services::{CheckinService, ConfigService, GroupService, MessageService, UserService, WsService},
    utils::{jwt::verify_jwt_token, response::AppError},
    ws::session::SessionManager,
};
use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::error;

pub struct AppState {
    pub config_service: Arc<ConfigService>,
    pub user_service: Arc<UserService>,
    pub group_service: Arc<GroupService>,
    pub message_service: Arc<MessageService>,
    pub checkin_service: Arc<CheckinService>,
    pub session_manager: Arc<SessionManager>,
    pub ws_service: Arc<WsService>,
}

// 认证中间件，用于验证请求中的JWT令牌
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AppError> {
    // 从路径判断是否需要认证
    let path = request.uri().path();

    // 如果是公开路径，直接放行
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // 获取认证头中的token
    let auth_header = match request.headers().get(header::AUTHORIZATION) {
        Some(header) => match header.to_str() {
            Ok(h) => h,
            Err(_) => return Err(AppError::Unauthorized("认证头格式无效".to_string())),
        },
        None => return Err(AppError::Unauthorized("缺少认证头".to_string())),
    };

    // 验证Bearer格式
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("认证头格式无效".to_string()));
    }

    // 提取token
    let token = auth_header.trim_start_matches("Bearer ").trim();
    if token.is_empty() {
        return Err(AppError::Unauthorized("令牌为空".to_string()));
    }

    // 验证token - 从config_service获取jwt_secret
    let jwt_secret = &state.config_service.config.jwt_secret;
    let claims = match verify_jwt_token(token, jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return Err(AppError::Unauthorized("令牌无效".to_string())),
    };

    // 获取用户ID
    let user_id = claims.sub;

    // 获取用户信息 - 使用专门用于认证的get_auth_info方法
    let user = state
        .user_service
        .get_auth_info(user_id)
        .await
        .map_err(|e| {
            error!("查询用户失败: {:?}", e);
            AppError::InternalServerError("服务器错误".to_string())
        })?;

    request.extensions_mut().insert(AuthUser {
        user_id: user.user_id,
        nickname: user.nickname,
    });

    // 执行请求
    Ok(next.run(request).await)
}

// 判断路径是否是公开路径（不需要认证）
fn is_public_path(path: &str) -> bool {
    // 从配置中获取公开路径列表
    crate::config::routes::get_public_routes()
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

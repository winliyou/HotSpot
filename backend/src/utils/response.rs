use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,       // 状态码，0表示成功，非0表示失败
    pub msg: String,     // 状态消息
    pub data: Option<T>, // 成功时为数据对象，失败时为null
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (error_message, error_code) = match self {
            AppError::NotFound(message) => (message, 1004), // 资源不存在
            AppError::BadRequest(message) => (message, 1000), // 参数验证错误
            AppError::InternalServerError(message) => (message, 5000), // 服务器内部错误
            AppError::Unauthorized(message) => (message, 1002), // 认证失败
            AppError::Forbidden(message) => (message, 1003), // 权限不足
            AppError::Conflict(message) => (message, 1005), // 冲突
        };

        // 所有错误响应统一使用HTTP状态码200，将实际错误码放在ApiResponse中
        let body = Json(ApiResponse::<()> {
            code: error_code,
            msg: error_message,
            data: None,
        });

        // 统一返回HTTP状态码200
        (StatusCode::OK, body).into_response()
    }
}

/// 创建带有自定义消息的成功响应
/// 设置code为0，自定义msg，并将数据放入data字段
pub fn success_response_with_msg<T: Serialize>(data: T, message: &str) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code: 0,
        msg: message.to_string(),
        data: Some(data),
    })
}

/// 创建标准成功响应
/// 设置code为0，msg为"success"，并将数据放入data字段
pub fn success_response<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code: 0,
        msg: "success".to_string(),
        data: Some(data),
    })
}

/// 通用分页参数
/// 用于所有需要分页功能的API请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// 每页限制数量，默认为20，最大50
    pub limit: Option<i64>,
    /// 分页游标，用于获取下一页数据
    pub cursor: Option<i64>,
}

/// 通用分页响应元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// 是否有更多数据
    pub has_more: bool,
    /// 下一页游标
    pub next_cursor: Option<i64>,
}

/// 通用分页响应结构体
/// 适用于所有需要分页的API响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    #[serde(flatten)]
    pub pagination: PaginationMeta,
}

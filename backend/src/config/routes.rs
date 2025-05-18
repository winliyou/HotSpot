/// 定义系统中的公开路由（不需要认证）
pub fn get_public_routes() -> Vec<&'static str> {
    vec![
        "/api/v1/user/register",
        "/api/v1/user/login",
        "/api/v1/user/create_temp_user",
        "/api/v1/user/refresh-token",
        "/ws/",
    ]
} 
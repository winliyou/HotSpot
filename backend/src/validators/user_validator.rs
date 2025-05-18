use crate::utils::response::AppError;

/// 验证用户注册信息
pub fn validate_user_registration(login_id: &str, password: Option<&str>, nickname: Option<&str>) -> Result<(), AppError> {
    // 验证登录ID
    if login_id.is_empty() || login_id.len() > 50 {
        return Err(AppError::BadRequest("登录ID长度应在1-50个字符之间".to_string()));
    }
    
    // 验证登录ID格式
    if !login_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err(AppError::BadRequest("登录ID只能包含字母、数字和_-.符号".to_string()));
    }
    
    // 验证密码
    if let Some(pwd) = password {
        if pwd.len() < 6 || pwd.len() > 20 {
            return Err(AppError::BadRequest("密码长度应在6-20个字符之间".to_string()));
        }
    }
    
    // 验证昵称
    if let Some(name) = nickname {
        if name.is_empty() || name.len() > 50 {
            return Err(AppError::BadRequest("昵称长度应在1-50个字符之间".to_string()));
        }
    }
    
    Ok(())
}

/// 验证昵称
pub fn validate_nickname(nickname: &str) -> Result<(), AppError> {
    if nickname.is_empty() || nickname.len() > 50 {
        return Err(AppError::BadRequest("昵称长度应在1-50个字符之间".to_string()));
    }
    Ok(())
}

/// 验证在线状态
pub fn validate_online_status(status: &str) -> Result<(), AppError> {
    match status {
        "online" | "offline" | "away" | "busy" => Ok(()),
        _ => Err(AppError::BadRequest(format!("无效的在线状态: {}", status))),
    }
} 
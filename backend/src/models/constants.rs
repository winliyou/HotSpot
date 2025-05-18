// 群组角色常量
pub mod group_roles {
    pub const MEMBER: &str = "member";
    pub const ADMIN: &str = "admin";
    pub const OWNER: &str = "owner";
    
    // 获取所有有效角色列表
    pub fn all_roles() -> Vec<&'static str> {
        vec![MEMBER, ADMIN, OWNER]
    }
    
    // 检查角色是否有效
    pub fn is_valid_role(role: &str) -> bool {
        all_roles().contains(&role)
    }
}

// 群组事件类型常量
pub mod group_event_types {
    pub const CREATE: &str = "create";
    pub const JOIN: &str = "join";
    pub const LEAVE: &str = "leave";
    pub const UPDATE: &str = "update";
    pub const KICK: &str = "kick";
    pub const TRANSFER: &str = "transfer";
    
    // 获取所有有效事件类型列表
    pub fn all_event_types() -> Vec<&'static str> {
        vec![CREATE, JOIN, LEAVE, UPDATE, KICK, TRANSFER]
    }
    
    // 检查事件类型是否有效
    pub fn is_valid_event_type(event_type: &str) -> bool {
        all_event_types().contains(&event_type)
    }
} 
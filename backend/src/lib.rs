//! HotSpot 后端库

// 导出模块供测试和其他组件使用
pub mod config;
pub mod controllers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;
pub mod validators;
pub mod ws;

// 重新导出主要组件用于测试
pub use config::db::Database;
pub use services::checkin_service::CheckinService;
pub use services::group_service::GroupService;
pub use services::message_service::MessageService;
pub use services::user_service::UserService;

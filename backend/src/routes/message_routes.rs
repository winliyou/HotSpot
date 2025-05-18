use axum::{Router, routing::post};

use crate::{controllers::message_controller, middleware::auth::AppState};
use std::sync::Arc;

pub fn message_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/group/send",
            post(message_controller::send_group_message),
        )
        .route(
            "/group/history",
            post(message_controller::group_history),
        )
        .route(
            "/group/delete",
            post(message_controller::delete_group_message),
        )
        .route("/user/send", post(message_controller::send_direct_message))
        .route(
            "/user/history",
            post(message_controller::user_history),
        )
        .route(
            "/user/conversations",
            post(message_controller::user_conversations),
        )
        .route(
            "/user/delete",
            post(message_controller::delete_direct_message),
        )
        .route(
            "/user/mark-read",
            post(message_controller::mark_messages_read),
        )
}

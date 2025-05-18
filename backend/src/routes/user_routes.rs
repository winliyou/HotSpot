use axum::{Router, routing::post};

use crate::{controllers::user_controller, middleware::auth::AppState};
use std::sync::Arc;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(user_controller::register))
        .route("/login", post(user_controller::login))
        .route("/create_temp_user", post(user_controller::create_temp_user))
        .route("/refresh-token", post(user_controller::refresh_token))
        .route(
            "/search_by_name",
            post(user_controller::search_user_by_name),
        )
        .route("/search_by_id", post(user_controller::search_user_by_id))
        .route(
            "/search_by_location",
            post(user_controller::search_user_by_location),
        )
        .route(
            "/update_location",
            post(user_controller::update_location),
        )
}

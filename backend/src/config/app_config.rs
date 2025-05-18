#[derive(Debug, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub database_url: String,
    pub redis_url: Option<String>,
    pub server_port: u16,
}

impl AppConfig {
    pub fn new(
        jwt_secret: String,
        jwt_expires_in: String,
        database_url: String,
        redis_url: Option<String>,
        server_port: u16,
    ) -> Self {
        Self {
            jwt_secret,
            jwt_expires_in,
            database_url,
            redis_url,
            server_port,
        }
    }
} 
use redis::Client as RedisClient;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Database {
    pub pg_pool: PgPool,
    pub redis_client: Option<Arc<RedisClient>>,
}

impl Database {
    pub async fn connect(
        database_url: &str,
        redis_url: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // 连接PostgreSQL
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Redis连接可选
        let redis_client = redis_url.map(|url| {
            let client = RedisClient::open(url).expect("无法创建Redis客户端");
            Arc::new(client)
        });

        Ok(Self {
            pg_pool,
            redis_client,
        })
    }
}

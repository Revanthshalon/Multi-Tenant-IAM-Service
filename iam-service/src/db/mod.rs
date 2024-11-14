use crate::config::AppConfig;
use sqlx::pool::PoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;

pub struct DbService {
    pool: PgPool,
}

impl DbService {
    pub fn new(app_config: Arc<AppConfig>) -> Self {
        let pool = PoolOptions::new()
            .max_connections(app_config.database_connection_pool_size())
            .idle_timeout(Duration::from_secs(
                app_config.database_connection_timeout(),
            ))
            .connect(app_config.database_url())
            .expect("Failed to create database connection pool");

        Self { pool }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}

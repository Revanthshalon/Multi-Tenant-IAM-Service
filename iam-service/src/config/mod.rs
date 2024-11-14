use std::sync::Arc;

pub struct AppConfig {
    database_url: String,
    database_connection_pool_size: u32,
    database_connection_timeout: u64,
    jwt_access_token_secret: String,
    jwt_access_token_duration: i64,
    jwt_refresh_token_secret: String,
    jwt_refresh_token_duration: i64,
}

impl AppConfig {
    pub fn load_config() -> Arc<Self> {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_connection_pool_size = env::var("DATABASE_CONNECTION_POOL_SIZE")
            .expect("DATABASE_CONNECTION_POOL_SIZE must be set")
            .parse::<u32>()
            .expect("DATABASE_CONNECTION_POOL_SIZE must be a number");
        let database_connection_timeout = env::var("DATABASE_CONNECTION_TIMEOUT")
            .expect("DATABASE_CONNECTION_TIMEOUT must be set")
            .parse::<u64>()
            .expect("DATABASE_CONNECTION_TIMEOUT must be a number");
        let jwt_access_token_secret =
            env::var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");
        let jwt_access_token_duration = env::var("JWT_ACCESS_TOKEN_DURATION")
            .expect("JWT_ACCESS_TOKEN_DURATION must be set")
            .parse::<i64>()
            .expect("JWT_ACCESS_TOKEN_DURATION must be a number");
        let jwt_refresh_token_secret =
            env::var("JWT_REFRESH_TOKEN_SECRET").expect("JWT_REFRESH_TOKEN_SECRET must be set");
        let jwt_refresh_token_duration = env::var("JWT_REFRESH_TOKEN_DURATION")
            .expect("JWT_REFRESH_TOKEN_DURATION must be set")
            .parse::<i64>()
            .expect("JWT_REFRESH_TOKEN_DURATION must be a number");

        let app_config = AppConfig {
            database_url,
            database_connection_pool_size,
            database_connection_timeout,
            jwt_access_token_secret,
            jwt_access_token_duration,
            jwt_refresh_token_secret,
            jwt_refresh_token_duration,
        };

        Arc::new(app_config)
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn database_connection_pool_size(&self) -> u32 {
        self.database_connection_pool_size
    }

    pub fn database_connection_timeout(&self) -> u64 {
        self.database_connection_timeout
    }

    pub fn jwt_access_token_secret(&self) -> &str {
        &self.jwt_access_token_secret
    }

    pub fn jwt_access_token_duration(&self) -> i64 {
        self.jwt_access_token_duration
    }

    pub fn jwt_refresh_token_secret(&self) -> &str {
        &self.jwt_refresh_token_secret
    }

    pub fn jwt_refresh_token_duration(&self) -> i64 {
        self.jwt_refresh_token_duration
    }
}

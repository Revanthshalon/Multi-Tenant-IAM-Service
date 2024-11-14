use std::sync::Arc;
use crate::config::AppConfig;
use crate::db::DbService;
use tokio::net::TcpListener;
use crate::repositories::RepositoryContainer;
use crate::routes::create_api_routes;
use crate::services::ServiceContainer;

pub mod config;
pub mod db;
pub mod entities;
pub mod errors;
pub mod handlers;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;

pub async fn run_iam_service(
    tcp_listener_option: Option<TcpListener>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::load_config();
    let db_service = DbService::new(app_config.clone());

    let repository_container = RepositoryContainer::new(db_service.pool());
    let service_container = ServiceContainer::new(repository_container);

    let app_state = AppState::new(service_container, app_config);

    let routes = create_api_routes(app_state);

    let tcp_listener = match tcp_listener_option {
        Some(listener) => listener,
        None => TcpListener::bind("localhost:5000").await.expect("Failed to bind to port 5000"),
    };

    axum::serve(tcp_listener, routes.into_make_service()).await.map_err(|err| err.into())
}

#[derive(Clone)]
pub struct AppState {
    pub service_container: Arc<ServiceContainer>,
    pub app_config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(service_container: Arc<ServiceContainer>, app_config: Arc<AppConfig>) -> Self {
        Self {
            service_container,
            app_config,
        }
    }
}
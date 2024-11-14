use crate::config::AppConfig;
use crate::db::DbService;
use tokio::net::TcpListener;
use crate::repositories::RepositoryContainer;
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

    let tcp_listener = match tcp_listener_option {
        Some(listener) => listener,
        None => TcpListener::bind("localhost:5000").await.expect("Failed to bind to port 5000"),
    };

    todo!()
}

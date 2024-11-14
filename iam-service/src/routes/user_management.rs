use axum::Router;
use crate::AppState;

pub fn create_user_management_routes(app_state: AppState) -> Router {
    Router::new()
        .with_state(app_state)
}
mod user_management;

use axum::Router;
use crate::AppState;
use crate::routes::user_management::create_user_management_routes;

pub fn create_api_routes(app_state: AppState) -> Router {

    let user_management_routes = create_user_management_routes(app_state.clone());

    let merged_routes = Router::new().merge(user_management_routes);

    let versioned_routes = Router::new()
        .nest("/v1", merged_routes);

    Router::new()
        .nest("/api", versioned_routes)
}
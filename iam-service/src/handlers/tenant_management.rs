use axum::{extract::State, response::Response, Json};

use crate::{models::tenant_management::RegisterTenant, AppState};

pub async fn register_tenant(
    State(app_state): State<AppState>,
    Json(user_details): Json<RegisterTenant>,
) -> Response {
    // let result = app_state.service_container.user_management_service.
    todo!()
}

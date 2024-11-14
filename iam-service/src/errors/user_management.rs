use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserManagementErrors {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Internal error: {0})")]
    DatabaseErrors(#[from] sqlx::Error),
}

impl IntoResponse for UserManagementErrors {
    fn into_response(self) -> Response {
        match self {
            UserManagementErrors::UserNotFound => {
                let status = StatusCode::NOT_FOUND;
                let response = Json(json!({
                    "error": "User not found"
                }));
                (status, response).into_response()
            }
            UserManagementErrors::UserAlreadyExists => {
                let status = StatusCode::CONFLICT;
                let response = Json(json!({
                    "error": "User already exists"
                }));
                (status, response).into_response()
            }
            UserManagementErrors::InternalError(message) => {
                // TODO: Log the error message
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let response = Json(json!({
                    "error": "Something went wrong",
                }));
                (status, response).into_response()
            }
            UserManagementErrors::DatabaseErrors(message) => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let response = Json(json!({
                    "error": "Something went wrong",
                }));
                (status, response).into_response()
            }
        }
    }
}

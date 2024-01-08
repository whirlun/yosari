pub mod app;
pub mod controllers;
pub mod mailers;
pub mod models;
pub mod tasks;
pub mod views;
pub mod workers;

use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
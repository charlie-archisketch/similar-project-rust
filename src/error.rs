use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{message}")]
    NotFound { message: String },
    #[error(transparent)]
    Internal {
        #[from]
        source: anyhow::Error,
    },
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl ApiError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    pub fn internal(source: impl Into<anyhow::Error>) -> Self {
        Self::Internal {
            source: source.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound { message } => {
                let body = Json(ErrorBody { message });
                (StatusCode::NOT_FOUND, body).into_response()
            }
            ApiError::Internal { source } => {
                tracing::error!(error = %source, "Unexpected internal failure");
                let body = Json(ErrorBody {
                    message: "unexpected error".to_string(),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

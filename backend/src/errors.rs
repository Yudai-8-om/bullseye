use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bullseye_api::errors::ScraperError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum BullsEyeError {
    #[error(transparent)]
    ScraperError(#[from] ScraperError),
    #[error(transparent)]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Database pooling connection error")]
    DbPoolError,
    #[error(transparent)]
    DateParseError(#[from] chrono::format::ParseError),
}

impl IntoResponse for BullsEyeError {
    fn into_response(self) -> Response {
        let status = match self {
            BullsEyeError::ScraperError(ScraperError::InvalidTickerError(_)) => {
                StatusCode::BAD_REQUEST
            }
            BullsEyeError::ScraperError(ScraperError::DriverFailureError(_)) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BullsEyeError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BullsEyeError::DbPoolError => StatusCode::INTERNAL_SERVER_ERROR,
            BullsEyeError::DateParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

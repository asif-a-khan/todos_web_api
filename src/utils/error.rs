use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    DatabaseError(sqlx::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
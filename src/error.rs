pub type Result<T> = core::result::Result<T, Error>;

use axum::{response::{IntoResponse, Response}, http::StatusCode};

#[derive(Debug)]
pub enum Error {
    LoginFail,
    NoAuthTokenCookie,
    WrongAuthTokenFormat
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
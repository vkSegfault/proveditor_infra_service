use crate::{error::Error, error::Result};
use serde::Deserialize;
use axum::{Json, routing::post, Router};
use serde_json::{Value, json};

pub fn crate_login_route() -> Router {
    Router::new()
        .route("/api/v1/login", post(api_login) )
}

async fn api_login( payload: Json<LoginPayload> ) -> Result<Json<Value>> {

    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    // TODO! set cookies

    // TODO! set success return body
    let body = Json(json!({
        "result" : {
            "success": true
        }
    }));

    todo!();
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}
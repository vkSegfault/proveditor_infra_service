use crate::error::{Error, Result};
use serde::Deserialize;
use axum::{Json, routing::post, Router};
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

pub const AUTH_TOKEN: &str = "auth-token";

pub fn create_auth_routers() -> Router {
    Router::new()
        .route("/api/v1/login", post(api_login) )
        .route("/api/v1/register", post(api_register) )
}

// JSON extractor must be last param
async fn api_login( cookies: Cookies, payload: Json<UserPayload> ) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO - implement DB check if user exists
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    // TODO! encode proper cookie
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // TODO! set success return body
    let body = Json(json!({
        "result" : {
            "success": true
        }
    }));

    // todo!();

    Ok(body)
}

async fn api_register( payload: Json<UserPayload> ) -> Result<Json<Value>> {
    // TODO! set success return body
    let body = Json(json!({
        "result" : {
            "success": true
        }
    }));

    // todo!();

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    username: String,
    password: String
}
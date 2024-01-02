use crate::error::{Error, Result};
use serde::Deserialize;
use axum::{Json, routing::post, routing::get, Router};
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};


pub fn create_auth_routers() -> Router {
    Router::new()
        .route("/api/v1/register", post(api_register) )
        .route("/api/v1/login", post(api_login) )
        .route("/api/v1/logout", get(api_logout) )
}


async fn api_register( payload: Json<UserPayload> ) -> Result<Json<Value>> {
    // TODO: create new user and add to DB
    // TODO! set success return body
    let body = Json(json!({
        "result" : {
            "success": "auth cookie added"
        }
    }));

    // todo!();

    Ok(body)
}


// JSON extractor must be last param
async fn api_login( cookies: Cookies, payload: Json<UserPayload> ) -> Result<Json<Value>> {
    // TODO: we can login with previousluy created new user or with SSO

    println!("->> {:<12} - api_login", "HANDLER");

    // TODO - implement DB check if user exists
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    // TODO! encode proper cookie
    // we set cookies if creds are valid from previous step
    println!( "Adding Auth Cookie..." );
    cookies.add(Cookie::new(crate::auth::AUTH_TOKEN, "user-1.exp.sign"));

    // TODO! set success return body
    let body = Json(json!({
        "result" : {
            "success": "auth cookie added"
        }
    }));

    Ok(body)
}

async fn api_logout( cookies: Cookies ) -> Result<Json<Value>> {

    cookies.remove( Cookie::from(crate::auth::AUTH_TOKEN) );

    let body = Json(json!({
        "result" : {
            "success": "auth cookie removed"
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    username: String,
    password: String
}
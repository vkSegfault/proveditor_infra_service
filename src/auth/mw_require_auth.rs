use axum::{response::Response, http::Request, middleware::Next};
use tower_cookies::Cookies;
use crate::error::Error;


pub async fn mw_require_auth<T>( cookies: Cookies, req: Request<T>, next: Next<T> ) -> Result<Response, Error>  {

    let auth_token = cookies.get( crate::auth::AUTH_TOKEN ).map(|c| c.value().to_string());
    
    match auth_token.ok_or( crate::error::Error::NoAuthTokenCookie ) {
        // what we actually do here is that we check if there is cookie named auth_token set, no any kind of validation
        
        // TODO - proper token parsing and validation
        Ok(token) => {
            println!( "Retrieved AUTH TOKEN from cookies: {token}" );
            Ok(next.run(req).await)
        } 
        Err(_) => {
            println!("ERROR: No Auth Cookie - try to login first");
            Err(Error::NoAuthTokenCookie)
        }
    }



}
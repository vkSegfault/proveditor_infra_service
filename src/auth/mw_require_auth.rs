use axum::{response::Response, http::Request, middleware::Next};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;
use crate::error::Error;


pub async fn mw_require_auth<T>( cookies: Cookies, req: Request<T>, next: Next<T> ) -> Result<Response, Error>  {

    let auth_token = cookies.get( crate::auth::AUTH_TOKEN ).map(|c| c.value().to_string());
    
    // check if token exists
    match auth_token.ok_or( crate::error::Error::NoAuthTokenCookie ) {
        
        // TODO - proper token parsing and validation
        Ok(token) => {
            println!( "Retrieved AUTH TOKEN from cookies: {token}" );

            // parse token into 3 parts
            match parse_token(token) {
                Ok( (user_id, expire, sign) ) => {

                    // TODO - user validation + expiration vlaidation + sign hash validation
                    
                    Ok(next.run(req).await)
                }
                Err(_) => {
                    Err(Error::WrongAuthTokenFormat)
                }
            }

        } 
        Err(_) => {
            println!("ERROR: No Auth Cookie - try to login first");
            Err(Error::NoAuthTokenCookie)
        }
    }
}

fn parse_token(token: String) -> Result<(u64, String, String), Error> {

    let (_whole_token, user_id, expire, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::WrongAuthTokenFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::WrongAuthTokenFormat)?;


    Ok( (user_id, expire.to_string(), sign.to_string()) )
}
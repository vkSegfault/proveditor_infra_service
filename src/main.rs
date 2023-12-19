mod repository;
mod service;
mod controller;
mod model;
mod error;
mod auth;
mod schema;

use std::net::SocketAddr;


#[tokio::main]
async fn main() {

    let infra_router = controller::create_routes();

    // use localhost in debug build
    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(( [127, 0, 0, 1], 8081 ));

    // run 0.0.0.0 in release mode (for Docker)
    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(( [0, 0, 0, 0], 8081 ));

    println!( "--> Listening on: {addr}" );

    axum::Server::bind( &addr )
        .serve(infra_router.into_make_service())
        .await
        .unwrap();

    println!("Server shutdown");

}
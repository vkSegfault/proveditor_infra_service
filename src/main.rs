mod repository;
mod service;
mod controller;
mod model;
mod error;
mod auth;
mod schema;

use std::net::SocketAddr;

use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), ()> {

    let infra_router = controller::create_router();

    // use localhost in debug build
    #[cfg(debug_assertions)]
    // let addr = SocketAddr::from(( [127, 0, 0, 1], 8081 ));  // old Axum 0.6
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();  // new Axum 0.7

    // run 0.0.0.0 in release mode (for Docker)
    #[cfg(not(debug_assertions))]
    // let addr = SocketAddr::from(( [0, 0, 0, 0], 8081 ));  // old Axum 0.6
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();  // new Axum 0.7

    println!( "--> Listening on: {listener:?}" );

    // OLD AXUM 0.6
    // axum::Server::bind( &addr )
    //     .serve(infra_router.into_make_service())
    //     .await
    //     .unwrap();

    // new Axum 0.7
    axum::serve(listener, infra_router.into_make_service() )
        .await
        .unwrap();

    println!("Server shutdown");

    Ok(())
}
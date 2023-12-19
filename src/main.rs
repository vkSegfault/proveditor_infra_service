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

    // TODO! execute it from service, not here
    // repository::connect_psql("user", "pass", "localhost", "5432", "mydb");

    let infra_router = controller::create_routes();

    let addr = SocketAddr::from(( [127, 0, 0, 1], 8081 ));
    println!( "--> Listening on: {addr}" );

    axum::Server::bind( &addr )
        .serve(infra_router.into_make_service())
        .await
        .unwrap();

    println!("Server shutdown");

}
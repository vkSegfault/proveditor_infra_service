// sudo apt install libpq-dev is needed for linker when using Postgres client
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn connect_psql<'a>( username: &'a str, password: &'a str, host: &'a str, port: &'a str, db_name: &'a str ) -> PgConnection {

    let connection_string = format!( "postgres://{username}:{password}@{host}:{port}/{db_name}" );
    let connection = PgConnection::establish(&connection_string).unwrap_or_else(|_| panic!("Error connecting to {connection_string}"));
    println!( "Connection to PostgreSQL established" );

    connection
}
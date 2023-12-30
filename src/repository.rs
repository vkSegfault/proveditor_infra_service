// sudo apt install libpq-dev is needed for linker when using Postgres client
extern crate diesel;
use diesel::expression::is_aggregate::No;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub struct PostgresConnector {
    pub conn: Option<PgConnection>
}

impl PostgresConnector {
    pub fn define_connection(&self) -> () {

    }

    pub fn get_connection(&self) -> Option<&PgConnection> {
        match &self.conn {
            Some(conn) => Some(conn),
            None => None
        }
    }
}

pub fn connect_psql<'a>( username: &'a str, password: &'a str, host: &'a str, port: &'a str, db_name: &'a str ) -> PgConnection {

    let connection_string = format!( "postgres://{username}:{password}@{host}:{port}/{db_name}" );
    let connection = PgConnection::establish(&connection_string).unwrap_or_else(|_| panic!("Error connecting to {connection_string}"));
    println!( "Connection to PostgreSQL established" );

    connection
}

pub fn get_connection<'a>() -> PgConnection {

    // TODO - fetch it from some env properties file instead hardcoding
    let username: &str = "user";
    let password: &str = "pass";
    #[cfg(debug_assertions)]  // use localhost in debug for local development
    let host: &str = "localhost";
    #[cfg(not(debug_assertions))]
    let host: &str = "postgresql";  // use Kuberenetes SVC to connect to Postgres in release
    let port: &str = "5432";
    let db_name: &str = "mydb";

    let connection_string = format!( "postgres://{username}:{password}@{host}:{port}/{db_name}" );
    let connection = PgConnection::establish(&connection_string).unwrap_or_else(|_| panic!("Error connecting to {connection_string}"));
    println!( "Connection to PostgreSQL established" );

    connection
}
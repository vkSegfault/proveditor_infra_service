use axum::body::Body;
use axum::Router;
use axum::extract::{Query, Path};
use axum::middleware::{Next, from_fn};
use axum::response::{Html, IntoResponse, Response, Json};
use axum::routing::get_service;
use axum::http::{StatusCode, header, Request};
use tower_http::services::ServeDir;
use tower_http::cors::{Any, CorsLayer};
use crate::model::Infra;
use crate::schema::infra;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(paths( post_handler, get_one_handler, get_all_handler, put_handler, delete_handler ), components(schemas( Infra )))]
struct ApiDoc;


pub fn create_routes() -> Router {
    let infra_router = Router::new();

    // CORS
    let cors = CorsLayer::new().allow_origin(Any);

    // POST
    let infra_router = infra_router.route("/api/v1/infra", axum::routing::post( post_handler ) );

    // GET one
    let infra_router = infra_router.route("/api/v1/infra/:name", axum::routing::get( get_one_handler ) );

    // GET all
    let infra_router = infra_router.route("/api/v1/infra", axum::routing::get( get_all_handler ) );

    // PUT
    let infra_router = infra_router.route("/api/v1/infra", axum::routing::put( put_handler ) );

    // DELETE
    let infra_router = infra_router.route("/api/v1/infra/:name", axum::routing::delete( delete_handler ) );

    println!( "Endpoints ready" );

    Router::new()
    .merge( infra_router )
    .merge( crate::auth::crate_login_route() )
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    .layer(cors)
    .layer(from_fn(logging_middleware))
    .fallback_service( serve_static_route() )  // if user provided endpoint that deosn't exists fallback to this static resource
}


fn serve_static_route() -> Router {
    // STATIC RESOURCE EXAMPLE: http://127.0.0.1:8080/src/main.rs - we can point to any static resource like .jpg or .txt

    println!( "Fallback main page" );

    Router::new().nest_service("/", get_service(ServeDir::new( "./" )))
}


// POST
#[utoipa::path(
    post, 
    path = "/api/v1/infra",
    request_body = Infra,
    responses(
        (status = 201, description = "Infra object created successfully"),
        (status = 409, description = "Infra object already exists")
    )
)]
async fn post_handler( Json(payload): Json<Infra> ) -> impl IntoResponse {
    println!( "POST request body: {payload:?}" );
    let conn = &mut crate::repository::connect_psql("user", "pass", "localhost", "5432", "mydb");
    
    let res = crate::service::create( payload, conn );

    match res {
        Some(object) => ( StatusCode::CREATED, [(header::CONTENT_TYPE, "application/json")], Json(format!("Created object: {object:?}")) ),
        None         => ( StatusCode::CONFLICT, [(header::CONTENT_TYPE, "application/json")], Json(format!("Infra object already exists")) )
    }
}


// GET ONE
#[utoipa::path(
    get,
    path = "/api/v1/infra/{name}",
    responses(
        (status = 200, description = "Infra object found", body = Infra),
        (status = 404, description = "Infra object not found")
    )
)]
// PATH PARAM EXAMPLE: http://127.0.0.1:8080/api/v1/infra/airport
async fn get_one_handler( Path(name): Path<String> ) -> impl IntoResponse {

    let conn = &mut crate::repository::connect_psql("user", "pass", "localhost", "5432", "mydb");
    let infra: Option<Infra> = crate::service::get_one( &name, conn);

    match infra {
        Some(infra) => ( StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], Json( format!("Found {infra:?}") ) ),
        None        => ( StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "application/json")], Json( format!("Infra object not found") ) )
    }
}


// GET ALL
#[utoipa::path(
    get,
    path = "/api/v1/infra",
    responses(
        (status = 200, description = "All Infra objects that were found", body = Vec<Infra>),
        (status = 404, description = "No Infra objects in DB")
    )
)]
async fn get_all_handler() -> impl IntoResponse {

    let conn = &mut crate::repository::connect_psql("user", "pass", "localhost", "5432", "mydb");
    let infras: Option<Vec<Infra>> = crate::service::get_all(conn);

    match infras {
        Some(infras) => {
            println!( "There are {} Infrastructute objects:", infras.len() );
            for i in &infras {
                println!( "{i:?}" );
            }
            ( StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], Json( format!("{infras:?}") ) )
        }
        None => ( StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "application/json")], Json(format!("No Infra objects in DB")) )
    }

}


// PUT
#[utoipa::path(
    put,
    path = "/api/v1/infra",
    params(
        Infra  // all fields from Infra struct wrapped into Optional<> are optional in Swagger as well
    ),
    responses(
        (status = 200, description = "Object updated successfuly", body = Infra),
        (status = 404, description = "Can't update object that doesn't exists")
    )
)]
// QUERY STRING EXAMPLE: http://localhost:8080/api/v1/infra?name=airport&price=2400&infra_modifier=0.3
async fn put_handler( Query(params): Query<Infra> ) -> impl IntoResponse {

    let conn = &mut crate::repository::connect_psql("user", "pass", "localhost", "5432", "mydb");
    let infra: Option<Infra> = crate::service::update(&params.name, params.infra_modifier, params.price, conn);

    match infra {
        Some(value) => {
            ( StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], Json( format!("Updated {0} to {value:?}", params.name) ) )
        },
        None => ( StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "application/json")], Json(format!("Can't update >> {0} << because it doesn't exists", params.name)) )
    }

}


// DELETE
#[utoipa::path(
    delete,
    path = "/api/v1/infra/{name}",
    responses(
        (status = 200, description = "Infra object deleted", body = Infra),
        (status = 404, description = "Infra object doesn't exists")
    )
)]
async fn delete_handler( Path(name): Path<String> ) -> impl IntoResponse {

    let conn = &mut crate::repository::connect_psql("user", "pass", "localhost", "5432", "mydb");
    let infra: Option<()> = crate::service::delete( &name, conn);

    match infra {
        Some(_) => ( StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], Json( format!("Deleted {name}") ) ),
        None        => ( StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "application/json")], Json( format!("Can't delete object {name} because it doesn't exists") ) )
    }
}


// LOGGING
async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!( "### LOGGER ### Received a {} request to {}", req.method(), req.uri() );
    next.run(req).await
}
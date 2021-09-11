use actix_cors::Cors;
use std::sync::Arc;

use actix_files as fs;
use actix_web::{
    http, middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use listenfd::ListenFd;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;

use crate::graphql_schema::{create_schema, Schema};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Earth! This is index()")
}

async fn api() -> impl Responder {
    HttpResponse::Ok().body("{\"status\": \"this sentence is in Rust\"}")
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    let path = std::env::current_dir()?;
    println!("The current directory is {}", path.display());

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/", web::get().to(index))
            .route("/api", web::get().to(api))
            .service(fs::Files::new("/", "../client/dist/").show_files_listing())
    });

    // This is the autoreload pattern from: https://actix.rs/docs/autoreload/
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        println!("Backend app running at http://localhost:3000/");
        server.bind("127.0.0.1:3000")?
    };

    // comand to run is
    // systemfd --no-pid -s http::3000 -- cargo watch -x run

    server.run().await
}

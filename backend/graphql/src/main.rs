//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web

use std::{env, io, net::SocketAddr, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware, route,
    web::{self, Data},
};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod schema;

use crate::schema::{Schema, create_schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|e| {
        info!(
            "Environmental variable not set. Setting default port to 4000 {}",
            e
        );
        String::from("4000")
    });
    let socket_address = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("./certs/local.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("./certs/local.crt")
        .unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 8081");
    log::info!("GraphiQL playground: http://localhost:8081/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind_openssl(socket_address, builder)?
    .run()
    .await
}

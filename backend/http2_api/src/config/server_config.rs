use std::env;
use std::net::SocketAddr;

use crate::api::v1::home::hello;
use crate::utils::certs::get_certs_config;
use axum::Router;
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(crate::api::v1::home::hello,))]
pub struct ApiDoc;

pub async fn configure_server() {
    let _ = dotenv_vault::dotenv();
    let rustls_config = get_certs_config().await;
    let app = Router::new().route("/api/v1", get(hello)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let mut server = axum_server::bind_rustls(addr, rustls_config);
    server.http_builder().http2().enable_connect_protocol();
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|e| {
        info!("ENVIRONMENT not set. Defaulting to production: {}", e);
        "PRODUCTION".to_string()
    });

    info!("Serving application at port :3000");
    if environment == "DEVELOPMENT" {
        info!(
            "Adding swagger api documentation at https://{}/swagger-ui",
            addr
        );
        let docs = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());
        let app = app.merge(docs);
        server.serve(app.into_make_service()).await.unwrap();
    } else {
        server.serve(app.into_make_service()).await.unwrap();
    }
}

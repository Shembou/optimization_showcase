use std::env;
use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::home::hello;
use crate::logging::logging::init_tracing;
use crate::utils::certs::get_certs_config;

#[derive(OpenApi)]
#[openapi(paths(crate::api::home::hello,))]
pub struct ApiDoc;

pub async fn configure_server() {
    init_tracing("logs", "app").expect("Failure during logger initialization");
    let rustls_config = get_certs_config().await;
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut server = axum_server::bind_rustls(addr, rustls_config);
    server.http_builder().http2().enable_connect_protocol();
    let arg = env::var("ENVIRONMENT");
    match arg {
        Ok(val) => {
            if val == "DEVELOPMENT" {
                let docs =
                    SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());
                let app = app.merge(docs);
                server.serve(app.into_make_service()).await.unwrap();
                return;
            }
        }
        Err(e) => info!("No env file. Setting configuration to production, {}", e),
    }
    server.serve(app.into_make_service()).await.unwrap();
}

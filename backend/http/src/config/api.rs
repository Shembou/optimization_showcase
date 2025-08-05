use std::{env, net::SocketAddr};

use tracing::info;

use crate::config::{certs::get_certs_config, router::configure_router};

pub async fn configure_rest_api() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Setting up routes");
    let router = configure_router();
    info!("binding socket address");
    let port = env::var("API_PORT").unwrap_or_else(|e| {
        info!(
            "API_PORT environment variable not set. Defaulting port to 3000. Err: {}",
            e
        );
        "3000".to_string()
    });
    let port_number: u16 = port.parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port_number));
    info!("Binding axum server");
    let certs = get_certs_config().await;
    let server = axum_server::bind_rustls(addr, certs);
    info!("Serving application at port :{}", port);

    server.serve(router.into_make_service()).await?;
    Ok(())
}
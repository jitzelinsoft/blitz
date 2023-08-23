use color_eyre::eyre::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::info;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)),3000);

    info!("Starting server on {}",socket);

    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

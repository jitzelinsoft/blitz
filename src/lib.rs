mod routes;
mod api;

use tracing::info;
use color_eyre::eyre::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub async fn run() -> Result<()> {
    let app = routes::get_router().merge(api::get_router()); 

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)),3000);

    info!("Starting server on {}",socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

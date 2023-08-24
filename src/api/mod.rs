mod agenda;

pub fn get_router() -> axum::Router {
    axum::Router::new()
        .route("/api/agenda", axum::routing::get(agenda::get))
}

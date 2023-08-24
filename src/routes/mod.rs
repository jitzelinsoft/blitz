mod layout;

pub fn get_router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(layout::get))
}

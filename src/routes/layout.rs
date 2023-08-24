use askama::Template;
use tracing::trace;

#[derive(Template)]
#[template(path="layout.html")]
pub struct LayoutTemplate {}

pub async fn get() -> LayoutTemplate {
    trace!("Someone visited the homepage");
    LayoutTemplate {} 
} 

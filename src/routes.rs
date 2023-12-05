use axum::{extract::DefaultBodyLimit, routing::post, Router};

mod handler;

pub fn init_pdf_routes() -> Router {
    const MAX_BODY_LIMIT: usize = 1000 * 1024 * 5;
    Router::new().route(
        "/",
        post(handler::pdf::handle_pdf_to_text).layer(DefaultBodyLimit::max(MAX_BODY_LIMIT)),
    )
}

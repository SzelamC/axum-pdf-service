use axum::Router;

mod routes;

static PORT: u32 = 3000;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new().merge(routes::init_pdf_routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .unwrap();
    println!("✅ Server is running on {PORT}");
    axum::serve(listener, app).await.unwrap();
}

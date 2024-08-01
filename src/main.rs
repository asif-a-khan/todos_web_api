use todos_web_api::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = routes::init::run().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod state;

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use interfaces::http_routes::build_router;
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("agora_api=debug".parse().unwrap())
            .add_directive("tower_http=info".parse().unwrap()))
        .init();

    let ollama_endpoint = std::env::var("OLLAMA_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    let default_model = std::env::var("OLLAMA_MODEL")
        .unwrap_or_else(|_| "qwen3:4b".to_string());
    let data_dir = std::env::var("DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());

    tracing::info!("Ollama endpoint: {}", ollama_endpoint);
    tracing::info!("Default model:   {}", default_model);
    tracing::info!("Data directory:  {}", data_dir);

    let state = AppState::new(ollama_endpoint, default_model, data_dir);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = build_router(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Agora API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

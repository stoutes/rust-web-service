
use axum::{routing::{get,post}, Router};
use std::net::SocketAddr;
use std::path::Path;
use tokio::fs;
use axum::{
    response::Html,
    http::StatusCode,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(say_hello_text))
        .route("/json", get(hello_json))
        .route("/post", post(say_post));
    

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}
async fn say_hello_text() -> Result<Html<String>, (StatusCode, String)> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(project_root)
        .join("static")
        .join("hello.html");
    eprintln!("‣ Looking for hello.html at: {}", path.display());
    match tokio::fs::metadata(&path).await {
        Ok(meta) => eprintln!("✔️  File found, size = {}", meta.len()),
        Err(err) => eprintln!("❌ metadata error: {}", err),
    };
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read file: {}", e),
            )
        })?;

    Ok(Html(content))
}
#[derive(Serialize)]
struct HelloJson{
    message: String,
}
async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson { message: "hi im JSON".to_string() };
    axum::Json(message)
}

async fn say_post() -> Result<Html<String>, (StatusCode, String)> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(project_root)
        .join("static")
        .join("hello.html");
    eprintln!("‣ Looking for hello.html at: {}", path.display());
    match tokio::fs::metadata(&path).await {
        Ok(meta) => eprintln!("✔️  File found, size = {}", meta.len()),
        Err(err) => eprintln!("❌ metadata error: {}", err),
    };
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read file: {}", e),
            )
        })?;

    Ok(Html(content))
}
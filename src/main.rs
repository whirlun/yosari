use axum::{
    routing::get,
    Router,
    http::{Request, StatusCode},
    response::{Json, Html, IntoResponse, Response},
    middleware::{Next, self},
    extract::Path, body::StreamBody,
};
use tokio_util::io::ReaderStream;
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer, Trace},
};

use glob::glob;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let app = Router
    ::new()
    .route("/", get(|| async { Html("Just Works") }))
    .route("/music/list", 
    get(file_list_handler))
    .route("/music/:music_name",
            get(music_handler))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
    //.layer(middleware::from_fn(csp_header));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
} 

async fn csp_header<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert("Content-Security-Policy", "connect-src 'self'".parse().unwrap());
    res
}

async fn music_handler(Path(music_name): Path<String>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/music/{music_name}")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND,())),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(body)
}

async fn file_list_handler() -> impl IntoResponse {
    let mut files = vec![];
        for p in glob("./storage/music/*").unwrap() {
            if let Ok(path) = p {
                files.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }        
    Json(files)
}
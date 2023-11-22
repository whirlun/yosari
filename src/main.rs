mod handlers;

use axum::{
    routing::{get, post},
    Router,
    http::{Request, StatusCode, HeaderMap, header},
    response::{Json, Html, IntoResponse, Response},
    middleware::{Next, self},
    extract::{Path, DefaultBodyLimit}, body::StreamBody,
};
use sea_orm::{DatabaseConnection, Database};
use tokio_util::io::ReaderStream;
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer, Trace},
};

use glob::glob;
use stretto::AsyncCache;
use std::net::SocketAddr;
use handlers::manga::manga_upload_handler;

#[derive(Clone)]
struct AppState {
    dbconn: DatabaseConnection,
    cache: AsyncCache<String, String>
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {

    let db = Database::connect("sqlite:////workspaces/yosari/dev.db").await.unwrap();
    let cache: AsyncCache<String, String> = AsyncCache::new(1024, 1e6 as i64, tokio::spawn).unwrap();
    let state = AppState{dbconn: db, cache};
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let app = Router
    ::new()
    .route("/", get(|| async { Html("It Just Works") }))
    .route("/upload/:manga_name/:file_name", get(upload_file_handler))
    .route("/music/list", get(file_list_handler))
    .route("/music/:music_name", get(music_handler))
    .route("/manga/:manga_name/page/:page", get(manga_handler))
    .route("/manga/:manga_name/pdf/:pdf_name", get(manga_pdf_handler))
    .route("/manga/upload", post(manga_upload_handler))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive())
    .layer(DefaultBodyLimit::max(1024*1000*1000*100))
    .with_state(state);
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
    let mut headers = HeaderMap::new();
    let body = StreamBody::new(stream);
    headers.insert(header::CONTENT_TYPE, "audio/mpeg".parse().unwrap());
    Ok((headers, body))
}

async fn manga_handler(Path((manga_name, page)): Path<(String, i32)>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/manga/{manga_name}/JPG/{page}.jpg")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpg".parse().unwrap());
    Ok((headers, body))
}

async fn manga_pdf_handler(Path((manga_name, pdf_name)): Path<(String, String)>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/manga/{manga_name}/{pdf_name}.pdf")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    Ok((headers, body))
}

async fn upload_file_handler(Path((manga_name, file_name)): Path<(String, String)>) -> impl IntoResponse {
    println!("{}", format!("./storage/upload/{manga_name}/{file_name}"));
    let file = match tokio::fs::File::open(format!("./storage/upload/{manga_name}/{file_name}")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpg".parse().unwrap());
    Ok((headers, body))
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
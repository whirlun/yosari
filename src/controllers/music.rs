use loco_rs::prelude::*;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, Json};
use glob::glob;
use tokio_util::io::ReaderStream;
use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::header;

pub async fn list() -> impl IntoResponse {
    let mut files = vec![];
        for p in glob("./storage/music/*").unwrap() {
            if let Ok(path) = p {
                files.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }        
    Json(files)
}

pub async fn get_music(Path(music_name): Path<String>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/music/{music_name}")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND,())),
    };
    let stream = ReaderStream::new(file);
    let mut headers = HeaderMap::new();
    let body = Body::from_stream(stream);
    headers.insert(header::CONTENT_TYPE, "audio/mpeg".parse().unwrap());
    Ok((headers, body))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("music")
        .add("/list", get(list))
        .add("/:music_name", get(get_music))
}

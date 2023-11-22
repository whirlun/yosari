use std::{
    io::Write,
    path::{PathBuf},
};

use crate::AppError;
use async_recursion::async_recursion;
use axum::{extract::{Multipart, Path}, http::StatusCode, response::IntoResponse, Json};
use entity::manga;
use infer::MatcherType;
use serde::Serialize;
use tokio::fs;
use walkdir::WalkDir;
use uuid::Uuid;

#[derive(Serialize)]
struct MangaUploadResponse {
    msg: String,
    images: Vec<String>,
    pdfs: Vec<String>,
}

impl Default for MangaUploadResponse {
    fn default() -> Self {
        MangaUploadResponse {
            msg: "".to_string(),
            images: vec![],
            pdfs: vec![],
        }
    }
}

pub async fn manga_upload_handler(mut multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    while let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name();
        if let Some(filename) = filename {
            if filename.ends_with(".zip") {
                let filename = filename.strip_suffix(".zip").unwrap().to_string();
                let filename2 = filename.clone();
                let mut temp = tempfile::tempfile()?;
                let bytes = file.bytes().await?;
                temp.write_all(&bytes)?;
                if let Ok((images, pdfs)) = tokio::task::spawn(async move {
                    let mut zip = zip::ZipArchive::new(temp)?;
                    zip.extract(format!("storage/upload/{filename}"))?;
                    let mut images: Vec<PathBuf> = vec![];
                    let mut pdfs: Vec<PathBuf> = vec![];
                    for entry in WalkDir::new(format!("storage/upload/{filename}")) {
                        if let Ok(entry) = entry {
                            if let Ok(Some(ft)) = infer::get_from_path(entry.path()) {
                                if let MatcherType::Image = ft.matcher_type() {
                                    images.push(entry.into_path());
                                } else if ft.mime_type() == "application/pdf" {
                                    pdfs.push(entry.into_path());
                                }
                            }
                        }
                    }
                    Ok::<(Vec<PathBuf>, Vec<PathBuf>), AppError>((images, pdfs))
                })
                .await?
                {
                    if images.is_empty() && pdfs.is_empty() {
                        fs::remove_dir_all(format! {"storage/upload/{filename2}"}).await?;
                        return Ok((
                            StatusCode::BAD_REQUEST,
                            Json(MangaUploadResponse {
                                msg: "nofile".to_string(),
                                ..Default::default()
                            }),
                        ));
                    } else {
                        return Ok((
                            StatusCode::ACCEPTED,
                            Json(MangaUploadResponse {
                                msg: Uuid::new_v4().to_string(),
                                images: images
                                    .iter()
                                    .map(|e| format!{"{filename2}/{}", e.file_name().unwrap().to_str().unwrap().to_owned()})
                                    .collect(),
                                pdfs: pdfs
                                    .iter()
                                    .map(|e| format!{"{filename2}/{}", e.file_name().unwrap().to_str().unwrap().to_owned()})
                                    .collect(),
                            }),
                        ));
                    }
                }
            }
        }
    }
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(MangaUploadResponse {
            msg: "error".to_string(),
            ..Default::default()
        }),
    ))
}

pub async fn finish_manga_upload_handler(trans_id: Path<String>) {
    
}
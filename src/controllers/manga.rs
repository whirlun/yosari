use loco_rs::prelude::*;
use std::{io::Write, path::PathBuf};

use axum::response::IntoResponse;
use axum::{extract::{Path, Multipart}, http::StatusCode, Json};
use tokio_util::io::ReaderStream;
use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::header;
use infer::MatcherType;
use pdfium_render::prelude::*;
use serde::Serialize;
use tokio::fs;
use uuid::Uuid;
use walkdir::WalkDir;
use crate::AppError;

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


pub async fn get_manga(Path((manga_name, page)): Path<(String, i32)>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/manga/{manga_name}/JPG/{page}.jpg")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpg".parse().unwrap());
    Ok((headers, body))
}

pub async fn manga_pdf_handler(Path((manga_name, pdf_name)): Path<(String, String)>) -> impl IntoResponse {
    let file = match tokio::fs::File::open(format!("./storage/manga/{manga_name}/{pdf_name}.pdf")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    Ok((headers, body))
}

pub async fn handler(Path((manga_name, file_name)): Path<(String, String)>) -> impl IntoResponse {
    println!("{}", format!("./storage/upload/{manga_name}/{file_name}"));
    let file = match tokio::fs::File::open(format!("./storage/upload/{manga_name}/{file_name}")).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, ())),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpg".parse().unwrap());
    Ok((headers, body))
}

pub async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse> {
    while let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name();
        if let Some(filename) = filename {
            if filename.ends_with(".zip") {
                let filename = filename.strip_suffix(".zip").unwrap().to_string();
                let filename2 = filename.clone();
                let mut temp = tempfile::tempfile().map_err(Error::wrap).map_err(Error::wrap)?;
                let bytes = file.bytes().await.map_err(Error::wrap)?;
                temp.write_all(&bytes).map_err(Error::wrap)?;
                match tokio::task::spawn(async move {
                    let mut zip = zip::ZipArchive::new(temp).map_err(Error::wrap)?;
                    zip.extract(format!("storage/upload/{filename}")).map_err(Error::wrap)?;
                    let mut images: Vec<PathBuf> = vec![];
                    let mut pdfs: Vec<PathBuf> = vec![];
                    for entry in WalkDir::new(format!("storage/upload/{filename}")) {
                        if let Ok(entry) = entry {
                            if let Ok(Some(ft)) = infer::get_from_path(entry.path()) {
                                if let MatcherType::Image = ft.matcher_type() {
                                    images.push(entry.into_path());
                                } else if ft.mime_type() == "application/pdf" {
                                    //export_pdf_to_jpegs(entry.path().to_path_buf()).await.map_err(Error::wrap)?;
                                    pdfs.push(entry.into_path());
                                }
                            }
                        }
                    }
                    Ok::<(Vec<PathBuf>, Vec<PathBuf>), Error>((images, pdfs))
                })
                .await
                {
                    Ok(Ok((images, pdfs))) => {
                        if images.is_empty() && pdfs.is_empty() {
                            fs::remove_dir_all(format! {"storage/upload/{filename2}"}).await.map_err(Error::wrap)?;;
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
                    Ok(Err(e)) => {
                        return Err(e);
                    }
                    _ => ()
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

async fn export_pdf_to_jpegs(path: PathBuf) -> Result<()> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library()).map_err(Error::wrap)?,
    );

    let document = pdfium.load_pdf_from_file(&path, None).map_err(Error::wrap)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);
    let storage_path = path.parent().unwrap().to_str().unwrap().to_owned()
        + "/"
        + path.file_stem().unwrap().to_str().unwrap();
    std::fs::create_dir_all(&storage_path).map_err(Error::wrap)?;
    for (index, page) in document.pages().iter().enumerate() {
        let format = page.render_with_config(&render_config).map_err(Error::wrap)?.format();
        page.render_with_config(&render_config)
        .map_err(Error::wrap)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)
            .map_err(Error::wrap)?
            .save_with_format(
                format!("{}/{index}.jpg", storage_path),
                image::ImageFormat::Jpeg,
            )
            .map_err(|_| Error::wrap(PdfiumError::ImageError))?;
    }
    Ok(())
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("manga")
        .add("/:manga_name/page/:page", get(get_manga))
        .add("/upload", post(upload))
}

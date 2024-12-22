use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};
use axum::{extract, Router};
use axum_extra::body::AsyncReadBody;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::fs;

#[derive(Clone, Debug)]
struct State {
    work_dir: Arc<String>,
    tg_api_url: Arc<String>,
}

#[tokio::main]
async fn main() {
    let state = State {
        work_dir: Arc::new("/data".to_string()),
        tg_api_url: Arc::new("http://127.0.0.1:8081".to_string()),
    };

    let app = Router::new()
        .route(
            "/file/*path",
            get(download)
        )
        .route("/*path", any(proxy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[axum_macros::debug_handler]
async fn proxy(extract::State(state): extract::State<State>,
                req: extract::Request) -> Response {
    let uri = req.uri().clone();

    let query_str = if let Some(q) = uri.query() {
        Cow::from(format!("?{}", q))
    } else {
        Cow::from("")
    };

    let target_url = format!("{}{}{}", state.tg_api_url.as_str(), uri.path(), query_str);

    let client = reqwest::Client::builder().build().expect("build reqwest client error");

    let Ok(resp) = client.request(req.method().clone(), target_url)
        .headers(req.headers().clone())
        .body(reqwest::Body::wrap_stream(req.into_body().into_data_stream()))
        .send().await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let mut response = Response::builder().status(resp.status());
    for (key, value) in resp.headers().iter() {
        response = response.header(key, value.clone());
    }
    let body = resp.bytes().await.unwrap();
    response.body(axum::body::Body::from(body)).unwrap()
}

#[axum_macros::debug_handler]
/// path format => bot{token}/{file_type}/{file_name}
/// example: bot123456:ABCDEF1234567890/stickers/file_0.webm
/// local mode path format => bot{token}/{work_dir}/{token}/{file_type}/{file_name}
/// example: bot123456:ABCDEF1234567890//data/123456:ABCDEF1234567890/stickers/file_0.webm
async fn download(
    extract::State(state): extract::State<State>,
    extract::Path(path): extract::Path<String>,
) -> Result<axum::response::Response<AsyncReadBody>, axum::response::Response> {
    // TODO should we check bot token valid?
    
    // get local absolute path
    let path = std::env::var("TELEGRAM_LOCAL_MODE")
        .map(|_| path.splitn(2, '/').nth(1).map(std::path::PathBuf::from))
        .unwrap_or_else(|_| path.strip_prefix("bot").map(|s| std::path::Path::new(state.work_dir.as_str()).join(s)));
    
    if let Some(path) = path {
        fs::try_exists(&path)
            .await
            .and(fs::File::open(&path).await)
            .map(move |file| {
                // let stream = tokio_util::io::ReaderStream::new(file);
                let body = axum_extra::body::AsyncReadBody::new(file);
                axum::response::Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        header::CONTENT_DISPOSITION,
                        format!(
                            "attachment; filename=\"{}\"",
                            path.file_name().map(|s| s.to_str().unwrap_or("")).unwrap_or("")
                        ),
                    )
                    .body(body)
                    .unwrap()
            })
            .map_err(|_| (StatusCode::NOT_FOUND, "file not found").into_response())
    } else {
        Err((StatusCode::NOT_FOUND, "file not found").into_response())
    }
}

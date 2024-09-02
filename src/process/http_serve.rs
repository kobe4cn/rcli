use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(_path: PathBuf, port: u16) -> anyhow::Result<()> {
    let state = HttpServeState {
        path: _path.clone(),
    };
    let dir_service = ServeDir::new(_path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    //axum router
    let app = Router::new()
        .route("/*path", get(file_handler))
        .route("/", get(file_handler_root))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));

    //start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let address = listener.local_addr().unwrap();
    info!("Server started at {}", address);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    //(StatusCode, String)
    println!("path: {:?}", path);
    // let path = match path {
    //     Some(p) => p,
    //     None => "./".to_string(),
    // };

    let p = std::path::Path::new(&state.path).join(path);
    info!("readering file: {:?}", &p);
    if p.exists() {
        if p.is_dir() {
            let mut entries = tokio::fs::read_dir(&p).await.unwrap();
            let mut content = String::new();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let c1 = format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    entry.path().display(),
                    entry.file_name().to_string_lossy()
                );
                content.push_str(&c1);
            }

            let content = format!("<html><body><ul>{}</ul></body></html>", content);
            return (StatusCode::OK, Html(content));
        }
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => {
                info!("file readded {} bytes: ", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(e) => {
                warn!("error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!("error reading file: {:?}", e)),
                )
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Html(format!("file not found: {:?}", &p)),
        )
    }
}

async fn file_handler_root(State(state): State<Arc<HttpServeState>>) -> (StatusCode, Html<String>) {
    // let path = match path {
    //     Some(p) => p,
    //     None => "./".to_string(),
    // };
    let path = "./";
    let p = std::path::Path::new(&state.path).join(path);
    info!("readering file: {:?}", &p);

    let mut entries = tokio::fs::read_dir(&p).await.unwrap();
    let mut content = String::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let c1 = format!(
            "<li><a href=\"{}\">{}</a></li>",
            entry.path().display(),
            entry.file_name().to_string_lossy()
        );
        content.push_str(&c1);
    }

    let content = format!("<html><body><ul>{}</ul></body></html>", content);
    (StatusCode::OK, Html(content))
}

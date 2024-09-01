use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::{info, warn};
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(_path: PathBuf, port: u16) -> anyhow::Result<()> {
    let state = HttpServeState { path: _path };
    //axum router
    let app = Router::new()
        .route("/*path", get(file_handler))
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
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("readering file: {:?}", &p);
    if p.exists() {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => {
                info!("file readded {} bytes: ", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("error reading file: {:?}", e),
                )
            }
        }
    } else {
        (StatusCode::NOT_FOUND, format!("file not found: {:?}", &p))
    }
}

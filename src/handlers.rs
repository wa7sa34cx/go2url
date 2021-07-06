use hyper::{Body, Request, Response, Result, StatusCode};
use crate::app;

/// Method Not Allowed
pub async fn method_not_allowed() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from("Method Not Allowed"))
        .unwrap())
}

/// File not found
pub async fn file_not_found() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("File not found"))
        .unwrap())
}

/// Internal Server Error
pub async fn error(e: &'static str) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(e))
        .unwrap())
}

/// Health-check
pub async fn health_check() -> Result<Response<Body>> {
    Ok(Response::new(Body::empty()))
}

/// Hello
pub async fn hello() -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello!")))
}

/// 301 redirect
pub async fn go(req: Request<Body>) -> Result<Response<Body>> {
    let filename = match req.uri().query() {
        Some(q) => q,
        None => return error("Expected filename in query").await,
    };

    let url = match app::get_line_from_db(filename) {
        Ok(line) => line,
        Err(e) => return error(e).await,
    };

    Ok(Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", url)
        .body(Body::empty())
        .unwrap())
}


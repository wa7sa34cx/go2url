use hyper::{Body, Request, Response, Result, StatusCode};

/// Page not found
pub async fn not_found() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page not found"))
        .unwrap())
}

/// File not found
pub async fn file_not_found() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("File not found"))
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

/// 301
pub async fn redirect(url: String) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", url)
        .body(Body::empty())
        .unwrap())
}

pub async fn go(req: Request<Body>) -> Result<Response<Body>> {
    let query = req.uri().query();
    match query {
        Some(q) => redirect(q.to_owned()).await,
        None => file_not_found().await,
    }
}
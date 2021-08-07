use crate::app;
use crate::validate;
use hyper::{Body, Request, Response, Result, StatusCode};

/// Method Not Allowed
pub async fn method_not_allowed() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from("Method Not Allowed"))
        .unwrap())
}

/// Internal Server Error
pub async fn error(e: String) -> Result<Response<Body>> {
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

/// 307 redirect
pub async fn go(req: Request<Body>) -> Result<Response<Body>> {
    let filename = match req.uri().query() {
        Some(q) => q,
        None => return error("Expected filename in query".to_owned()).await,
    };

    if !validate::txt_file(filename) {
        return error("Invalid file name. Expected: example.txt".to_owned()).await;
    }

    let url = match app::get_rand_line_from_db(filename).await {
        Ok(line) => line,
        Err(e) => return error(e.to_string()).await,
    };

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", url)
        .body(Body::empty())
        .unwrap())
}

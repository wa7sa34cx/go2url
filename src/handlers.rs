use crate::app;
use crate::error::AppError;
use crate::validate;
use hyper::{Body, Request, Response, Result, StatusCode};
use std::{error, thread, time::Duration};

/// Method Not Allowed
pub async fn method_not_allowed() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from("Method Not Allowed"))
        .unwrap())
}

/// Internal Server Error
pub async fn error<T>(e: T) -> Result<Response<Body>>
where
    T: error::Error,
{
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(e.to_string()))
        .unwrap())
}

/// Health-check
pub async fn health_check() -> Result<Response<Body>> {
    Ok(Response::new(Body::empty()))
}

/// Long operation
pub async fn long() -> Result<Response<Body>> {
    thread::sleep(Duration::from_secs(10));
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
        None => return error(AppError::EmptyQuery).await,
    };

    if !validate::txt_file(filename) {
        return error(AppError::NotValidQuery).await;
    }

    let url = match app::get_rand_line_from_db(filename).await {
        Ok(line) => line,
        Err(e) => return error(e).await,
    };

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", url)
        .body(Body::empty())
        .unwrap())
}

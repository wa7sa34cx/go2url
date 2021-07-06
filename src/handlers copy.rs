use crate::validate;
use crate::db::DB;
use dotenv::dotenv;
use std::env;
use hyper::{Body, Request, Response, Result, StatusCode};

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

/// 301
// pub async fn redirect(filename: String) -> Result<Response<Body>> {
//     dotenv().ok();
//     let db_path = match env::var("DB_PATH") {
//         Ok(s) => s,
//         Err(e) => return error("DB_PATH must be set in .env").await,
//     };

//     let path = db_path + "one-line.txt";

//     // hello().await

//     // Ok(Response::builder()
//     //     .status(StatusCode::MOVED_PERMANENTLY)
//     //     .header("Location", url)
//     //     .body(Body::empty())
//     //     .unwrap())
// }

pub async fn go(req: Request<Body>) -> Result<Response<Body>> {
    let query = req.uri().query();
    match query {
        None => error("Expected filename in query").await,
        Some(q) => {
            if !validate::txt_file(q) {
                return error("Invalid file name. Expected: example.txt").await;
            }

            redirect(q.to_owned()).await
        }
    }
}

// redirect(q.to_owned()).await,

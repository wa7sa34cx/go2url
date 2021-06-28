mod handlers;

use hyper::{Body, Method, Request, Response, Result};

pub async fn make(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => handlers::hello().await,
        (&Method::GET, "/health_check") => handlers::health_check().await,
        (&Method::GET, "/go") => handlers::go(req).await,
        _ => handlers::not_found().await,
    }
}


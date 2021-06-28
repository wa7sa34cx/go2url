use hyper::{Body, Method, Request, Response, Result, StatusCode};

pub async fn make(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => hello().await,
        (&Method::GET, "/health_check") => health_check().await,
        (&Method::GET, "/go") => go(req).await,
        _ => not_found().await,
    }
}

/// Page not found
async fn not_found() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page not found"))
        .unwrap())
}

/// File not found
async fn file_not_found() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("File not found"))
        .unwrap())
}

/// Health-check
async fn health_check() -> Result<Response<Body>> {
    Ok(Response::new(Body::empty()))
}

/// Hello
async fn hello() -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello!")))
}

async fn go(req: Request<Body>) -> Result<Response<Body>> {
    let query = req.uri().query();
    match query {
        Some(q) => Ok(Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", q.to_owned())
            .body(Body::empty())
            .unwrap()),

        // Ok(Response::new(Body::from(q.to_owned()))),
        None => file_not_found().await,
    }
}

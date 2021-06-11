use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};
use std::net::SocketAddr;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

static INDEX: &str = "examples/send_file_index.html";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            simple_file_send(INDEX).await
        },
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        },
        (&Method::GET, "/health_check") => {
            health_check().await
        },
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}

/// Health-check endpoint
async fn health_check() -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Ok")))
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}

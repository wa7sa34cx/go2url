mod service;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(move |req| service::make(req)))
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

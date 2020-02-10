use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use tokio::task::JoinError;

mod asset;

async fn serve(req: Request<Body>) -> Result<Response<Body>, JoinError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let toppage = asset::Asset::async_get("index.html").await?.unwrap();
            Ok(Response::new(toppage.into()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_service = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(serve))
    });
    let server = hyper::Server::bind(&addr).serve(make_service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

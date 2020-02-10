use hyper::{Body, Method, Request, Response, StatusCode, Error};
use super::asset;

pub async fn routes(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let toppage = asset::WWW::async_get("index.html").await;
            Ok(Response::new(toppage.into()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}

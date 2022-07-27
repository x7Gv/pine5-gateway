use futures::future;
use hyper::{body, Body, Request, Response, Server};
use tower::Service;

const ROOT: &str = "/";
const STATUS: &str = "/status";

#[derive(Debug)]
pub struct Api;

impl Service<Request<Body>> for Api {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();

        let uri = req.uri();

        match uri.path() {
            STATUS => {
                let body = Body::from(Vec::from(&b"status"[..]));
                let rsp = rsp.status(200).body(body).unwrap();
                return future::ok(rsp);
            }
            &_ => {
                let body = Body::from(Vec::new());
                let rsp = rsp.status(404).body(body).unwrap();
                return future::ok(rsp);
            }
        }
    }
}

pub struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = Api;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: T) -> Self::Future {
        future::ok(Api)
    }
}

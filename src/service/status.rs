use hyper::Request;
use tower::Service;

enum StatusRequest<S, P>
where
    S: Service<tonic::Request<>>,
    P: FnMut(&S) -> bool,
{
    Default,
    Filter(P),
}

pub struct ServiceStatus;

impl<S: Service, P: FnMut(&S) -> bool> Service<Request<StatusRequest<>>> {

}

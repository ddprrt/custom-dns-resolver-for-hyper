use std::{
    future::Future,
    io::{self, ErrorKind},
    pin::Pin,
    task::Poll,
};

use hyper::{
    client::connect::dns::{GaiAddrs, GaiResolver, Name},
    service::Service,
};
use std::net::ToSocketAddrs;

use crate::has_localhost::{HasLocalhost};

#[derive(Clone)]
pub struct MyResolver {
    inner: GaiResolver,
}

impl MyResolver {
    fn new(inner: GaiResolver) -> Self {
        Self { inner }
    }
}

impl Default for MyResolver {
    fn default() -> Self {
        Self::new(GaiResolver::new())
    }
}

impl Service<Name> for MyResolver {
    type Response = GaiAddrs;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Name) -> Self::Future {
        let addr = req.as_str();
        let addr = (addr, 0).to_socket_addrs();     

        if let Ok(_) = addr.map(|mut el| el.has_localhost()) {
            return Box::pin(async {
                let err = io::Error::from(ErrorKind::Other);
                Err(err)
            });
        }

        let ft = self.inner.call(req);
        Box::pin(ft)
    }
}

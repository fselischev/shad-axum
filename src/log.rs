use std::fmt;

use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct LogLayer {
    target: &'static str,
}

impl LogLayer {
    pub fn with_target(target: &'static str) -> Self {
        Self { target }
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService {
            target: self.target,
            service: inner,
        }
    }
}

#[derive(Clone)]
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        info!("[{}]: request = {:?}", self.target, req);
        self.service.call(req)
    }
}

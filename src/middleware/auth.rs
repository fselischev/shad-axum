use axum::{
    http::{Request, StatusCode},
    response::IntoResponse,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use tower::{Layer, Service};

static VALID_TOKEN: &str = "VALID";

#[derive(Clone)]
pub(crate) struct AuthChecker;

impl AuthChecker {
    pub async fn check_token(&self, token: &str) -> Result<(), &'static str> {
        if token == VALID_TOKEN {
            Ok(())
        } else {
            Err("Invalid token")
        }
    }
}

#[derive(Clone)]
pub struct AuthLayer {
    checker: Arc<AuthChecker>,
}

impl AuthLayer {
    pub fn new() -> Self {
        Self {
            checker: Arc::new(AuthChecker {}),
        }
    }
}

impl<S: Clone> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            checker: self.checker.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
    checker: Arc<AuthChecker>,
}

impl<S> Service<Request<axum::body::Body>> for AuthService<S>
where
    S: Service<Request<axum::body::Body>, Response = axum::response::Response>
        + Send
        + Clone
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<axum::body::Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let checker = self.checker.clone();
        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {
            if let Some(value) = auth_header {
                if let Ok(value_str) = value.to_str() {
                    let mut parts = value_str.split_whitespace();
                    if let (Some(scheme), Some(token)) = (parts.next(), parts.next()) {
                        if scheme == "Bearer" {
                            match checker.check_token(token).await {
                                Ok(()) => {
                                    return inner.call(req).await;
                                }
                                Err(_) => {
                                    return Ok(StatusCode::UNAUTHORIZED.into_response());
                                }
                            }
                        }
                    }
                }
            }

            Ok(StatusCode::UNAUTHORIZED.into_response())
        })
    }
}

use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};

use async_graphql::{
    Executor,
    http::{create_multipart_mixed_stream, is_accept_multipart_mixed},
};
use axum::{
    BoxError,
    body::{Body, HttpBody},
    extract::FromRequest,
    http::{Request as HttpRequest, Response as HttpResponse},
    response::IntoResponse,
};
use bytes::Bytes;
use futures_util::{StreamExt, future::BoxFuture};
use tower_service::Service;

use crate::{
    GraphQLBatchRequest, GraphQLRequest, GraphQLResponse, extract::rejection::GraphQLRejection,
};

/// A GraphQL service.
#[derive(Clone)]
pub struct GraphQL<E> {
    executor: E,
}

impl<E> GraphQL<E> {
    /// Create a GraphQL handler.
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<B, E> Service<HttpRequest<B>> for GraphQL<E>
where
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    E: Executor,
{
    type Response = HttpResponse<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HttpRequest<B>) -> Self::Future {
        let executor = self.executor.clone();
        let req = req.map(Body::new);
        Box::pin(async move {
            let is_accept_multipart_mixed = req
                .headers()
                .get("accept")
                .and_then(|value| value.to_str().ok())
                .map(is_accept_multipart_mixed)
                .unwrap_or_default();

            if is_accept_multipart_mixed {
                let req = match GraphQLRequest::<GraphQLRejection>::from_request(req, &()).await {
                    Ok(req) => req,
                    Err(err) => return Ok(err.into_response()),
                };
                let stream = executor.execute_stream(req.0, None);
                let body = Body::from_stream(
                    create_multipart_mixed_stream(stream, Duration::from_secs(30))
                        .map(Ok::<_, std::io::Error>),
                );
                Ok(HttpResponse::builder()
                    .header("content-type", "multipart/mixed; boundary=graphql")
                    .body(body)
                    .expect("BUG: invalid response"))
            } else {
                let req =
                    match GraphQLBatchRequest::<GraphQLRejection>::from_request(req, &()).await {
                        Ok(req) => req,
                        Err(err) => return Ok(err.into_response()),
                    };
                Ok(GraphQLResponse(executor.execute_batch(req.0).await).into_response())
            }
        })
    }
}

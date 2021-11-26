use tonic::transport::Body;

pub struct JsonRpc {

}

impl tonic::codegen::Service<tonic::codegen::http::Request<Body>> for JsonRpc {
    type Response = tonic::codegen::http::Response<tonic::body::BoxBody>;
    type Error = tonic::codegen::Never;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: tonic::codegen::http::Request<Body>) -> Self::Future {
        Box::pin(async move {
            Ok(tonic::codegen::http::Response::builder()
                .status(200)
                .header("grpc-status", "100")
                .header("content-type", "application/grpc")
                .body(tonic::body::BoxBody::default())
                .unwrap())
        })
    }
}
impl  tonic::transport::NamedService for JsonRpc {
    const NAME: &'static str = "";
}
impl Clone for JsonRpc {
    fn clone(&self) -> Self {
        Self {
        }
    }
}

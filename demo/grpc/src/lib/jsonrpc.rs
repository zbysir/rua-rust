use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::transport::Body;
use hyper;
use hyper::body::Bytes;

pub struct JsonRpc<S> {
    // 当前 server
    server: S,
    // 判断当前 server 是否满足执行
    // dyn Future<Output=bool>
    attch: Arc<dyn (Fn(&Request<Body>) -> bool) + Send + Sync+ 'static>,
    // 如果当前 server 无法满足，则执行 next
    next: Option<Arc<Self>>,
}

impl<S> tonic::codegen::Service<tonic::codegen::http::Request<Body>> for JsonRpc<S>
    where
        S: tonic::codegen::Service<tonic::codegen::http::Request<Body>, Response=tonic::codegen::http::Response<tonic::body::BoxBody>> + Clone + Send + 'static,
        S::Future: Send + 'static,
{
    type Response = tonic::codegen::http::Response<tonic::body::BoxBody>;
    type Error = tonic::codegen::Never;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: tonic::codegen::http::Request<Body>) -> Self::Future {
        Box::pin(async move {
            // println!("self.attch: {:?}", (*self.attch)(&req).await);

            Ok(tonic::codegen::http::Response::builder()
                .status(200)
                .header("grpc-status", "100")
                .header("content-type", "application/grpc")
                .body(tonic::body::BoxBody::default())
                .unwrap())
        })
    }
}

use serde_derive::Deserialize;
use tonic::codegen::http::Request;

#[derive(Debug, Deserialize)]
struct JsonRpcReq {
    method: String,
    params: Box<serde_json::value::RawValue>,
}

impl<S, > JsonRpc<S>
    where
        S: tonic::codegen::Service<tonic::codegen::http::Request<Body>, Response=tonic::codegen::http::Response<tonic::body::BoxBody>>
        + tonic::transport::NamedService
        + Clone
        + Send
        + 'static,
        S::Future: Send + 'static,

// S::Error: Into<crate::Error> + Send,
{
    pub fn new(svc: S) -> Self {
        let svc_name = <S as tonic::transport::NamedService>::NAME;
        let svc_route = format!("/{}", svc_name);
        println!("svc_route: {:?}", svc_route);

        // let pred = move |req: &tonic::codegen::http::Request<Body>| async {
        //     let body: Body = req.into_body();
        //     let bytes: Bytes = hyper::body::to_bytes(body).await.unwrap();
        //     let json_req: JsonRpcReq = serde_json::from_reader(bytes.reader()).unwrap();
        //
        //     println!("req: {:?}", json_req);
        //     true
        // };
        let pred= move|req: &Request<Body>| {
                let body = req.body();
                // let bytes: Bytes = hyper::body::to_bytes(body).await.unwrap();
                // let json_req: JsonRpcReq = serde_json::from_reader(bytes).unwrap();
                // println!("json_req: {:?}", bytes);
                println!("req: {:?}", req);
                true
        };
        JsonRpc {
            server: svc,
            attch: Arc::new(pred),
            next: None,
        }
    }
    // pub fn add_service(self, svc: S) -> Self
    // {
    //     let svc_name = <S as tonic::transport::NamedService>::NAME;
    //     let svc_route = format!("/{}", svc_name);
    //     println!("svc_route: {:?}", svc_route);
    //
    //     // let pred = move |req: &tonic::codegen::http::Request<Body>| async {
    //     //     let body: Body = req.into_body();
    //     //     let bytes: Bytes = hyper::body::to_bytes(body).await.unwrap();
    //     //     let json_req: JsonRpcReq = serde_json::from_reader(bytes.reader()).unwrap();
    //     //
    //     //     println!("req: {:?}", json_req);
    //     //     true
    //     // };
    //     let pred = move |req: &Request<Body>|  async{
    //         println!("req: {:?}", req);
    //         true
    //     };
    //
    //         JsonRpc {
    //         server: svc,
    //         attch: Arc::new(pred),
    //         next: Some(Arc::new(self)),
    //     }
    // }
}


impl<S> tonic::transport::NamedService for JsonRpc<S> {
    const NAME: &'static str = "";
}

impl<S> Clone for JsonRpc<S>
    where S: Clone
{
    fn clone(&self) -> Self {
        Self {
            attch: self.attch.clone(),
            server: self.server.clone(),
            next: self.next.clone(),
        }
    }
}

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::transport::Body;
use hyper;
use hyper::body::Bytes;

/// JsonRpc 转换 jsonrpc 协议 为 grpc+json。让服务同时支持两种协议。
pub struct JsonRpc<S> {
    servers: Vec<S>,
}

use std::fmt::Debug;

impl<S> tonic::codegen::Service<tonic::codegen::http::Request<Body>> for JsonRpc<S>
    where
        S: tonic::codegen::Service<tonic::codegen::http::Request<Body>, Response=tonic::codegen::http::Response<tonic::body::BoxBody>> + Clone + Send + 'static,
        S::Future: Send + 'static,
        S::Error: Into<StdError> + Send + 'static + Debug,
{
    type Response = tonic::codegen::http::Response<tonic::body::BoxBody>;
    type Error = tonic::codegen::Never;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: tonic::codegen::http::Request<Body>) -> Self::Future {
        let mut s = self.servers[0].clone();
        Box::pin(async move {
            let body = req.into_body();
            let bytes: Bytes = hyper::body::to_bytes(body).await.unwrap();
            let json_req: JsonRpcReq = serde_json::from_slice(bytes.as_ref()).unwrap();

            let p = json_req.params[0].get();
            let mut params_str = "\x00\x00\x00\x00".to_string();
            let l = p.len();
            params_str.push(if l > u8::MAX as usize { 0 } else { l as u8 } as char);
            params_str.push_str(&p.to_string());
            println!("req: {:?}", params_str);

            let request: tonic::codegen::http::Request<tonic::transport::Body> = tonic::codegen::http::Request::builder()
                .method("GET")
                .uri("/helloworld.Job/TriggerCreateRebate")
                .header("X-Custom-Foo", "Bar")
                .body(tonic::transport::Body::from(params_str))
                .unwrap();

            let response = (s).call(request).await;
            let r: tonic::codegen::http::Response<tonic::body::BoxBody> = response.unwrap();
            let body = r.into_body();
            let bytes: Bytes = hyper::body::to_bytes(body).await.unwrap();
            let mut bytes:Bytes = bytes.slice(5..);

            println!("rsp: {:?}", bytes);
            use http_body::Body;

            use std::str;
            let mut string:String =str::from_utf8(bytes.as_ref()).unwrap().into();
            let rsp_json =  serde_json::to_string(&JsonRpcRsp{
                id: json_req.id,
                result: serde_json::value::RawValue::from_string(string).unwrap()
            }).unwrap();

            println!("rsp_json: {:?}", rsp_json);

            let rsp_body = http_body::combinators::UnsyncBoxBody::new(tonic::transport::Body::from(rsp_json).
                map_err(|err| tonic::Status::ok("44")));

            Ok(tonic::codegen::http::Response::builder()
                .status(200)
                .header("grpc-status", "0")
                .header("content-type", "application/json")
                .body(rsp_body)
                .unwrap())
        })
    }
}

use serde_derive::Deserialize;
use serde_derive::Serialize;
use tonic::codegen::http::Request;
use tonic::codegen::StdError;

#[derive(Debug, Deserialize)]
struct JsonRpcReq {
    method: String,
    params: Vec<Box<serde_json::value::RawValue>>,
    id : String,
}

#[derive(Debug, Serialize)]
struct JsonRpcRsp {
    result: Box<serde_json::value::RawValue>,
    id : String,
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

        let mut x = Vec::new();
        x.push(svc);
        JsonRpc {
            servers: x
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
            servers: self.servers.clone(),
        }
    }
}

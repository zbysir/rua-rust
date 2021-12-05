use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::transport::Body;
use hyper;
use hyper::body::Bytes;

// 一个简单版的 grpc 路由，为了同时支持多种 url 格式，如 /.HelloWorld/SayHello 和 /helloworld.HelloWorld/SayHello。
pub struct ServiceRouter<S> {
    server_map: HashMap<String, S>,
}

use std::fmt::Debug;

impl<S> tonic::codegen::Service<tonic::codegen::http::Request<Body>> for ServiceRouter<S>
    where
        S: tonic::codegen::Service<tonic::codegen::http::Request<Body>, Response=tonic::codegen::http::Response<tonic::body::BoxBody>>
        + Clone
        + Send
        + tonic::transport::NamedService
        + 'static,
        S::Future: Send + 'static,
        S::Error: Into<StdError> + Send + 'static + Debug,
{
    type Response = tonic::codegen::http::Response<tonic::body::BoxBody>;
    type Error = NeverErr;
    // type Error = Box<dyn std::error::Error + 'static>;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: tonic::codegen::http::Request<Body>) -> Self::Future {
        let path = req.uri().path();
        let (service, _) = match_uri(path);

        println!("xxx {:?}", service);
        match self.server_map.get(service) {
            Some(s) => {
                let mut s = s.clone();
                Box::pin(async move {
                    let x = s.call(req).await;
                    match x {
                        Ok(o) => Ok(o),
                        Err(e) => {
                            Err(NeverErr {})
                        }
                    }
                })
            }
            None => {
                Box::pin(async move {
                    return Ok(tonic::codegen::http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/json")
                        .body(empty_body())
                        .unwrap());
                })
            }
        }
    }
}

fn match_uri(path: &str) -> (&str, &str) {
    let ss = path.split(".").collect::<Vec<&str>>();
    let mut ser_method: (&str, &str) = ("", "");
    if ss.len() > 1 {
        let sx: Vec<&str> = ss.last().unwrap().split("/").collect();
        ser_method = (sx[0], sx[1]);
    } else {
        let sx: Vec<&str> = ss[0].trim_start_matches("/").split("/").collect();
        ser_method = (sx[0], sx[1]);
    }

    ser_method
}

fn match_service(path: &str, named_service: &str) -> bool {
    let (ser, _) = match_uri(path);
    let ss = named_service.split(".").collect::<Vec<&str>>();
    ss.last().unwrap() == &ser
}

fn get_service_name(named_service: &str) -> &str {
    let ss = named_service.split(".").collect::<Vec<&str>>();
    ss.last().unwrap()
}

#[derive(Debug)]
pub struct NeverErr {}

impl std::fmt::Display for NeverErr {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for NeverErr {}


use serde_derive::Deserialize;
use serde_derive::Serialize;
use tonic::body::empty_body;
use tonic::codegen::http::Request;
use tonic::codegen::{Never, StdError};

impl<S, > ServiceRouter<S>
    where
        S: tonic::codegen::Service<tonic::codegen::http::Request<Body>, Response=tonic::codegen::http::Response<tonic::body::BoxBody>>
        + tonic::transport::NamedService
        + Clone
        + Send
        + 'static,
        S::Future: Send + 'static,
{
    pub fn new(svc: S) -> Self {
        ServiceRouter {
            server_map: HashMap::new(),
        }.add_service(svc)
    }
    pub fn add_service(mut self, svc: S) -> Self {
        let svc_name = <S as tonic::transport::NamedService>::NAME;
        let service_name = get_service_name(svc_name);

        println!("add_service {:?}", service_name);
        self.server_map.insert(service_name.to_string(), svc);
        ServiceRouter {
            server_map: self.server_map,
        }
    }
}

impl<S> tonic::transport::NamedService for ServiceRouter<S> {
    const NAME: &'static str = "";
}

impl<S> Clone for ServiceRouter<S>
    where S: Clone
{
    fn clone(&self) -> Self {
        Self {
            server_map: self.server_map.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grpc_router::match_uri;

    #[test]
    fn t_match_uri() {
        assert_eq!(match_uri("/helloworld.Job/TriggerCreateRebate"), ("Job", "TriggerCreateRebate"));
        assert_eq!(match_uri("/.Job/TriggerCreateRebate"), ("Job", "TriggerCreateRebate"));
        assert_eq!(match_uri("/Job/TriggerCreateRebate"), ("Job", "TriggerCreateRebate"));
    }
}
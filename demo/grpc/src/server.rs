use std::marker::PhantomData;
use std::task::{Context, Poll};
use prost::bytes::{Buf, BufMut};
use tonic::{transport::Server, Request, Response, Status};
use tonic::body::BoxBody;
use tonic::Code::{Internal, InvalidArgument};
use tonic::codec::{Codec, DecodeBuf, Decoder, Encoder, EncodeBuf};
use tonic::codegen::BoxFuture;
use tonic::transport::Body;

use hello_world::job_server::{Job, JobServer};
use hello_world::{HelloReply, HelloRequest};

extern crate serde;

extern crate serde_derive;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

struct JsonEncoder<T>(PhantomData<T>);

impl<T: serde::Serialize> Encoder for JsonEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let s = serde_json::to_string(&item).unwrap();
        buf.put(s.as_bytes());

        Ok(())
    }
}

pub struct JsonDecoder<U>(PhantomData<U>);

impl<U: for<'a> serde::Deserialize<'a>> Decoder for JsonDecoder<U> {
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let item = match serde_json::from_reader(buf.reader()) {
            Ok(i) => i,
            Err(e) => {
                return Err(Status::new(tonic::Code::Internal, e.to_string()));
            }
        };
        Ok(item)
    }
}

#[derive(Debug, Clone)]
struct JsonCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<T, U> Default for JsonCodec<T, U> {
    fn default() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T, U> Codec for JsonCodec<T, U>
    where
        T: serde::Serialize + Send + 'static,
        U: for<'a> serde::Deserialize<'a> + Send + Default + 'static,
{
    type Encode = T;
    type Decode = U;
    type Encoder = JsonEncoder<T>;
    type Decoder = JsonDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        JsonEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        JsonDecoder(PhantomData)
    }
}


#[derive(Debug, Default, Clone)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Job for MyGreeter {
    async fn trigger_create_rebate(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

mod lib;

use lib::jsonrpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;
    // let greeter = ;
    let mut s = JobServer::new(MyGreeter::default());

    let json = jsonrpc::JsonRpc::new(s.clone());

    Server::builder()
        .accept_http1(true)
        .add_service(json)
        .add_service(s)
        .serve(addr)
        .await?;


    Ok(())
}

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use serde::Serialize;

pub trait Caller<Req, Rsp> {
    fn call(&self, req: tonic::Request<Req>) -> BoxFuture<tonic::Response<Rsp>, tonic::Status>;
}

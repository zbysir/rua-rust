use std::future::Future;
use std::sync::Arc;
use std::pin::Pin;

struct X {
    s: Arc<dyn Fn(i32) -> Pin<Box<dyn Future<Output=i32>>>>,
}

#[tokio::main]
async fn main() {
    let pred:Arc<dyn Fn(i32) -> Pin<Box<dyn Future<Output=i32>>>> = Arc::new(move |req: i32| {
        Box::pin(async move {
            req + 1
        })
    });

    let x = X {
        s: pred,
    };

    println!("{:?}", (x.s)(1).await);
}
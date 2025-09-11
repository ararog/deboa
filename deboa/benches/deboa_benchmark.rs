#[cfg(feature = "tokio-rt")]
use tokio::runtime::Runtime;

#[cfg(feature = "smol-rt")]
use criterion::async_executor::SmolExecutor;

use criterion::{Criterion, criterion_group, criterion_main};
use deboa::{Deboa, errors::DeboaError, request::DeboaRequest};

async fn get_async() -> Result<(), DeboaError> {
    let mut api = Deboa::new();
    let _ = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts").send_with(&mut api).await;
    Ok(())
}

async fn post_async() -> Result<(), DeboaError> {
    let mut api = Deboa::new();
    let _ = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")
        .text("Some test to do")
        .send_with(&mut api)
        .await;
    Ok(())
}

fn deboa(c: &mut Criterion) {
    c.bench_function("deboa_get", move |b| {
        #[cfg(feature = "tokio-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let _ = get_async().await;
        });

        #[cfg(feature = "smol-rt")]
        b.to_async(SmolExecutor).iter(|| async {
            let _ = get_async().await;
        });
    });

    c.bench_function("deboa_post", move |b| {
        #[cfg(feature = "tokio-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let _ = post_async().await;
        });

        #[cfg(feature = "smol-rt")]
        b.to_async(SmolExecutor).iter(|| async {
            let _ = post_async().await;
        });
    });
}

criterion_group!(benches, deboa);

criterion_main!(benches);

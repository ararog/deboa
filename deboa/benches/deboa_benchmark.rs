#[cfg(feature = "tokio-rt")]
use tokio::runtime::Runtime;

#[cfg(feature = "smol-rt")]
use criterion::async_executor::SmolExecutor;

use criterion::{criterion_group, criterion_main, Criterion};
use deboa::{request::DeboaRequest, Deboa, Result};

async fn get_async() -> Result<()> {
    let api = Deboa::new();
    let _ = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts")?
        .go(api)
        .await;
    Ok(())
}

async fn post_async() -> Result<()> {
    let api = Deboa::new();
    let _ = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")?
        .text("Some test to do")
        .go(api)
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

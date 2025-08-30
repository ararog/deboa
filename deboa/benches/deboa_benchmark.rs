#[cfg(feature = "compio-rt")]
use criterion::async_executor::SmolExecutor;
#[cfg(feature = "tokio-rt")]
use tokio::runtime::Runtime;

#[cfg(feature = "smol-rt")]
use criterion::async_executor::SmolExecutor;

#[cfg(feature = "compio-rt")]
use criterion::async_executor::CompioExecutor;

use criterion::{criterion_group, criterion_main, Criterion};
use deboa::{errors::DeboaError, Deboa};

async fn get_async() -> Result<(), DeboaError> {
    let api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    let _ = api.get("/posts").await;
    Ok(())
}

async fn post_async() -> Result<(), DeboaError> {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    let _ = api.set_text("Some test to do".to_owned()).post("/posts").await;
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

        #[cfg(feature = "compio-rt")]
        b.to_async(CompioExecutor).iter(|| async {
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

        #[cfg(feature = "compio-rt")]
        b.to_async(CompioExecutor).iter(|| async {
            let _ = post_async().await;
        });
    });
}

criterion_group!(benches, deboa);

criterion_main!(benches);

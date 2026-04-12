use deboa_tokio::Client;

use criterion::{criterion_group, criterion_main, Criterion};
use deboa::{request::DeboaRequest, Result};
use tokio::runtime::Runtime;

async fn get_async() -> Result<()> {
    let client = Client::default();
    let _res = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts")?
        .send_with(&client)
        .await;

    Ok(())
}

async fn post_async() -> Result<()> {
    let client = Client::default();
    let _ = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts")?
        .text("Some test to do")
        .send_with(&client)
        .await;
    Ok(())
}

fn deboa(c: &mut Criterion) {
    c.bench_function("deboa_get", move |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let _ = get_async().await;
            });
    });

    c.bench_function("deboa_post", move |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| async {
                let _ = post_async().await;
            });
    });
}

criterion_group!(benches, deboa);

criterion_main!(benches);

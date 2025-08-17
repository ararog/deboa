use criterion::{criterion_group, criterion_main, Criterion};
use deboa::Deboa;
use serde::Serialize;
use tokio::runtime::Runtime;

#[derive(Serialize)]
struct Post {
    id: u64,
    title: String,
    body: String,
}

async fn get_async() {
    let api = Deboa::new("https://jsonplaceholder.typicode.com");
    let _ = api.get("/posts").await;
}

async fn post_async() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    let _ = api.set_json(Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    }).post("/posts").await;
}

fn deboa(c: &mut Criterion) {
    c.bench_function("deboa_get", move |b| {
        #[cfg(feature = "tokio-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            get_async().await;
        });

        #[cfg(feature = "smol-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            get_async().await;
        });
    });

    c.bench_function("deboa_post", move |b| {
        #[cfg(feature = "tokio-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            post_async().await;
        });

        #[cfg(feature = "smol-rt")]
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            post_async().await;
        });
    });
}

criterion_group!(benches, deboa);

criterion_main!(benches);

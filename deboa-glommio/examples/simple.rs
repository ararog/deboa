//! One request on a glommio executor.
//!
//! ```sh
//! cargo run -p deboa-glommio --example simple -- https://example.com
//! ```

use deboa::request::{get, FetchWith};
use deboa_glommio::Client;

fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".to_string());

    // glommio has no `#[main]` attribute, so the executor is explicit.
    // `LocalExecutorPoolBuilder` runs one per core when you want the machine.
    glommio::LocalExecutorBuilder::default()
        .make()
        .expect("build glommio executor")
        .run(async move {
            let client = Client::default();

            // deboa defaults a request to HTTP/2; ask for 1.1 when this build
            // has no http2 feature, or the dispatcher has no protocol to use.
            #[cfg(not(feature = "http2"))]
            let request = get(url.as_str())
                .unwrap()
                .version(http::Version::HTTP_11);
            #[cfg(feature = "http2")]
            let request = get(url.as_str()).unwrap();

            match request.send_with(&client).await {
                Ok(response) => println!("{}", response.status()),
                Err(e) => {
                    eprintln!("request failed: {e:?}");
                    std::process::exit(1);
                }
            }
        });
}

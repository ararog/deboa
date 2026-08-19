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
            match get(url.as_str()).unwrap().send_with(&client).await {
                Ok(response) => println!("{}", response.status()),
                Err(e) => {
                    eprintln!("request failed: {e:?}");
                    std::process::exit(1);
                }
            }
        });
}

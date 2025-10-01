use clap::Parser;
use deboa::{errors::DeboaError, Deboa};
use deboa_macros::{fetch, submit};
use tokio::io::{self, AsyncWriteExt};

#[derive(Parser)]
#[command(name = "rurl", about = "rurl - a cli tool to make http requests", long_about = None)]
struct Args {
    #[arg(short, long, required = true, help = "The URL to make the request to.")]
    url: String,
    #[arg(short, long, help = "The HTTP method to use.")]
    method: String,
    #[arg(short, long, help = "The body of the request.")]
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let args = Args::parse();
    let mut client = Deboa::new();
    let response = if args.method == "GET" || args.method == "DELETE" {
        fetch!(args.url, &mut client)
    } else {
        let method = args.method.parse();
        if let Err(e) = method {
            eprintln!("Error: {}", e);
            return Err(DeboaError::ProcessResponse {
                message: "Invalid HTTP method".to_string(),
            });
        }
        submit!(method.unwrap(), args.body.as_str(), args.url, &mut client)
    };
    let mut stdout = io::stdout();
    let result = stdout.write(response.raw_body()).await;
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        return Err(DeboaError::Io {
            message: "Failed to write to stdout".to_string(),
        });
    }

    Ok(())
}

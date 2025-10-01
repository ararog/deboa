use clap::Parser;
use deboa::{errors::DeboaError, request::DeboaRequest, Deboa};
use deboa_macros::fetch;
use http::{HeaderMap, HeaderName, HeaderValue};
use tokio::io::{self, AsyncWriteExt};

#[derive(Parser)]
#[command(name = "rurl", about = "rurl - a cli tool to make http requests", long_about = None)]
struct Args {
    #[arg(short, long, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(short, long, help = "HTTP method to use.")]
    method: String,
    #[arg(short, long, help = "Request body.")]
    body: String,
    #[arg(short, long, help = "Request headers.")]
    headers: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let args = Args::parse();
    let mut client = Deboa::new();

    let method = args.method.parse();
    if let Err(e) = method {
        eprintln!("Error: {}", e);
        return Err(DeboaError::ProcessResponse {
            message: "Invalid HTTP method".to_string(),
        });
    }
    let headers = args.headers.iter().fold(HeaderMap::new(), |mut map, header| {
        let pairs = header.split_once(':');
        if let Some((key, value)) = pairs {
            let header_name = HeaderName::from_bytes(key.as_bytes());
            if let Err(e) = header_name {
                eprintln!("Invalid header name: {}", e);
                return map;
            }
            let header_value = HeaderValue::from_bytes(value.as_bytes());
            if let Err(e) = header_value {
                eprintln!("Invalid header value: {}", e);
                return map;
            }
            map.append(header_name.unwrap(), header_value.unwrap());
        }
        map
    });
    
    let request = DeboaRequest::at(args.url, method.unwrap())?.headers(headers);
    let request = if args.method == "GET" || args.method == "DELETE" {
        request
    } else {
        request.text(&args.body)
    };
    let response = client.execute(request.build()?).await?;

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

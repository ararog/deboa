use clap::Parser;
use deboa::{errors::DeboaError, request::DeboaRequest, Deboa};
use http::{header, HeaderMap, HeaderName, HeaderValue, Method};
use tokio::io::{self, AsyncWriteExt};

#[derive(Parser)]
#[command(name = "rurl", about = "rurl - a cli tool to make http requests", long_about = None)]
struct Args {
    #[arg(short, long, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(short, long, help = "HTTP method to use.")]
    method: Option<String>,
    #[arg(short, long, help = "Request body.")]
    body: Option<String>,
    #[arg(long, help = "Request header.")]
    header: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let args = Args::parse();
    let mut client = Deboa::new();

    let method = args.method.unwrap_or("GET".to_string()).parse::<Method>();
    if let Err(e) = method {
        eprintln!("Error: {}", e);
        return Err(DeboaError::ProcessResponse {
            message: "Invalid HTTP method".to_string(),
        });
    }

    let mut headers = if let Some(header) = args.header {
        header.iter().fold(HeaderMap::new(), |mut map, header| {
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
        })
    } else {
        HeaderMap::new()
    };

    let http_method = method.unwrap();
    let request = DeboaRequest::at(args.url, http_method.clone())?;
    let request = if (http_method == Method::GET || http_method == Method::DELETE) && args.body.is_none() {
        request
    } else if let Some(body) = args.body {
        let content_length = body.len();
        headers.insert(
            header::CONTENT_LENGTH,
            HeaderValue::from_bytes(content_length.to_string().as_bytes()).unwrap(),
        );
        if http_method == Method::GET {
            request.method(Method::POST).text(&body)
        } else {
            request.text(&body)
        }
    } else {
        request
    };
    let request = request.headers(headers);
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

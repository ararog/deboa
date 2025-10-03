use clap::Parser;
use deboa::{errors::DeboaError, form::Form, request::DeboaRequest, Deboa};
use http::{header, HeaderMap, HeaderName, HeaderValue, Method};
use tokio::io::{self, AsyncWriteExt};

#[derive(Parser)]
#[command(
    name = "uget",
    about = "uget - a cli tool to make http requests",
    long_about = r#"
uget - a cli tool to make http requests

Usage:
    uget <URL> [OPTIONS]

Options:
    -h, --help       Print help information
    -V, --version    Print version information
    -m, --method <METHOD>
                     HTTP method to use
    -b, --body   <BODY>
                     Allow set raw request body
    -f, --field <FIELD>
                     Set form field, format: key=value
    -H, --header <HEADER>
                     Set request header field, format: key:value
    -B, --bearer <BEARER>
                     Set bearer auth token on Authorization header
    -a, --basic  <BASIC>
                     Set basic auth on Authorization header, format: username:password, it will be base64 encoded
"#
)]
struct Args {
    #[arg(short, long, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(short, long, help = "HTTP method to use.")]
    method: Option<String>,
    #[arg(short, long, help = "Allow set raw request body.")]
    body: Option<String>,
    #[arg(long, help = "Set form field, format: key=value.")]
    field: Option<Vec<String>>,
    #[arg(long, help = "Set header field, format: key:value.")]
    header: Option<Vec<String>>,
    #[arg(long, help = "Set bearer auth token on Authorization header.")]
    bearer: Option<String>,
    #[arg(
        long,
        help = "Set basic auth on Authorization header, format: username:password, it will be base64 encoded."
    )]
    basic: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut client = Deboa::new();

    let result = handle_request(&args, &mut client).await;
    if let Err(err) = result {
        eprintln!("An error occurred: {:#}", err);
    }
}

async fn handle_request(args: &Args, client: &mut Deboa) -> Result<(), DeboaError> {
    let mut arg_url = args.url.clone();
    let arg_method = args.method.as_ref();
    let arg_body = args.body.as_ref();
    let arg_fields = args.field.as_ref();
    let arg_header = args.header.as_ref();
    let arg_bearer_auth = args.bearer.as_ref();
    let arg_basic_auth = args.basic.as_ref();
    let method = arg_method.unwrap_or(&"GET".to_string()).parse::<Method>();
    if let Err(e) = method {
        eprintln!("Error: {}", e);
        return Err(DeboaError::ProcessResponse {
            message: "Invalid HTTP method".to_string(),
        });
    }

    if arg_body.is_some() && arg_fields.is_some() {
        eprintln!("Error: Both body and fields are set");
        return Err(DeboaError::ProcessResponse {
            message: "Both body and fields are set".to_string(),
        });
    }

    if arg_url.starts_with(":") {
        let port = arg_url.strip_prefix(":");
        if let Some(port) = port {
            if port.starts_with('/') {
                arg_url = format!("http://localhost{}", port);
            } else {
                arg_url = format!("http://localhost:{}", port);
            }
        }
    }

    let mut headers = if let Some(header) = arg_header {
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
    let request = DeboaRequest::at(arg_url, http_method.clone())?;
    let request = if (http_method == Method::GET || http_method == Method::DELETE) && args.body.is_none() {
        request
    } else if let Some(body) = arg_body {
        let content_length = body.len();
        headers.insert(
            header::CONTENT_LENGTH,
            HeaderValue::from_bytes(content_length.to_string().as_bytes()).unwrap(),
        );
        if http_method == Method::GET {
            request.method(Method::POST).text(body)
        } else {
            request.text(body)
        }
    } else if let Some(fields) = arg_fields {
        let mut form = Form::builder();
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        for field in fields {
            let pairs = field.split_once('=');
            if let Some((key, value)) = pairs {
                form.field(key.to_string(), value.to_string());
            }
        }
        request.text(&form.build())
    } else {
        request
    };
    let request = request.headers(headers);

    let request = if let Some(bearer_auth) = arg_bearer_auth {
        request.bearer_auth(bearer_auth)
    } else {
        request
    };

    let request = if let Some(basic_auth) = arg_basic_auth {
        let (username, password) = basic_auth.split_once(':').unwrap();
        request.basic_auth(username, password)
    } else {
        request
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

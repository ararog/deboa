use std::{
    borrow::Cow,
    io::{Read, Write},
};

use clap::Parser;
use deboa::{
    errors::DeboaError,
    form::{DeboaForm, EncodedForm},
    request::DeboaRequest,
    Deboa, Result,
};
use http::{header, HeaderMap, HeaderName, HeaderValue, Method};
use std::fs::File;
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
    -f, --field  <FIELD>
                     Set form field, format: key=value
    -H, --header <HEADER>
                     Set request header field, format: key=value
    -B, --bearer <BEARER>
                     Set bearer auth token on Authorization header
    -a, --basic  <BASIC>
                     Set basic auth on Authorization header, format: username=password, it will be base64 encoded
    -s, --save   <FILE_PATH>
                     Set the file to save the response body.
    -p, --part   <PART>
                     Set the part of multipart/form-data.
    -b, --bdry   <BOUNDARY>
                     Set boundary for multipart/form-data.
    -r, --raw    <RAW>
                     Set raw request body.
"#
)]
struct Args {
    #[arg(index = 1, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(short, long, help = "HTTP method to use.")]
    method: Option<String>,
    #[arg(short, long, help = "Allow set raw request body.")]
    body: Option<String>,
    #[arg(long, help = "Set form field, format: key=value.")]
    field: Option<Vec<String>>,
    #[arg(long, help = "Set header field, format: key=value.")]
    header: Option<Vec<String>>,
    #[arg(long, help = "Set bearer auth token on Authorization header.")]
    bearer: Option<String>,
    #[arg(
        long,
        help = "Set basic auth on Authorization header, format: username=password, it will be base64 encoded."
    )]
    basic: Option<String>,
    #[arg(long, help = "Set the file to save the response body.")]
    save: Option<String>,
    #[arg(long, help = "Set the part of multipart/form-data.")]
    part: Option<Vec<String>>,
    #[arg(long, help = "Set boundary for multipart/form-data.")]
    bdry: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut client = Deboa::new();

    let result = handle_request(args, &mut client).await;
    if let Err(err) = result {
        eprintln!("An error occurred: {:#}", err);
    }
}

async fn handle_request(args: Args, client: &mut Deboa) -> Result<()> {
    let mut arg_url = args.url;
    let arg_method = args.method;
    let mut arg_body = args.body;
    let arg_fields = args.field;
    let arg_header = args.header;
    let arg_bearer_auth = args.bearer;
    let arg_basic_auth = args.basic;
    let arg_save = args.save;
    let arg_part = args.part;
    let _arg_bdry = args.bdry;

    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    let stdin_body = reader.read_to_string(&mut buffer);
    if let Err(e) = stdin_body {
        return Err(DeboaError::Io {
            message: format!("Failed to read from stdin: {}", e),
        });
    }

    if !buffer.is_empty() {
        arg_body = Some(buffer);
    }

    let mut method = Cow::from("GET");
    if let Some(some_method) = arg_method {
        method = some_method.to_uppercase().into();
    }

    let method = method.parse::<Method>();
    if let Err(e) = method {
        return Err(DeboaError::ProcessResponse {
            message: format!("Invalid HTTP method: {}", e),
        });
    }

    if arg_body.is_some() && arg_fields.is_some() && arg_part.is_some() {
        return Err(DeboaError::ProcessResponse {
            message: "Both body, fields and part are set, you can only use one of them.".to_string(),
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
    let request = DeboaRequest::to(arg_url.as_ref())?;
    let request = if (http_method == Method::GET || http_method == Method::DELETE) && arg_body.is_none() && arg_fields.is_none() && arg_part.is_none()
    {
        request
    } else if let Some(body) = arg_body {
        let content_length = HeaderValue::from_str(&body.len().to_string());
        headers.insert(header::CONTENT_LENGTH, content_length.unwrap());
        if http_method == Method::GET {
            request.method(Method::POST).text(&body)
        } else {
            request.text(&body)
        }
    } else if let Some(fields) = arg_fields {
        let mut form = EncodedForm::builder();
        let content_type = mime::APPLICATION_WWW_FORM_URLENCODED.as_ref();
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_str(content_type).unwrap());
        for field in fields {
            let pairs = field.split_once('=');
            if let Some((key, value)) = pairs {
                form.field(key, value);
            }
        }
        request.text(&form.build())
    } else {
        request
    };
    let request = request.headers(headers);

    let request = if let Some(bearer_auth) = arg_bearer_auth {
        request.bearer_auth(&bearer_auth)
    } else {
        request
    };

    let request = if let Some(basic_auth) = arg_basic_auth {
        let (username, password) = basic_auth.split_once(':').unwrap();
        request.basic_auth(username, password)
    } else {
        request
    };

    let request = request.method(http_method);

    let response = client.execute(request.build()?).await?;

    if let Some(save) = arg_save {
        let file = File::create(save);
        if let Ok(mut file) = file {
            let result = file.write(response.raw_body());
            if let Err(e) = result {
                return Err(DeboaError::Io {
                    message: format!("Failed to write to file: {}", e),
                });
            }
        }
    } else {
        let mut stdout = io::stdout();
        let result = stdout.write(response.raw_body()).await;
        if let Err(e) = result {
            return Err(DeboaError::Io {
                message: format!("Failed to write to stdout: {}", e),
            });
        }
    }

    Ok(())
}

use std::io::{stdin, IsTerminal, Read, Write};

use clap::Parser;
use deboa::{
    cert::ClientCert,
    errors::DeboaError,
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    Deboa, Result,
};
use http::{header, HeaderName, Method};
use std::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[derive(Parser)]
#[command(
    name = "uget",
    about = "uget - a cli tool to make http requests",
    long_about = r#"
uget - a cli tool to make http requests

Usage:
    uget <URL> <BODY> [OPTIONS]

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
    -c, --cert   <CERT>
                     Set the certificate file to use.
    -k, --key    <KEY>
                     Set the private key file to use.
    -K, --key-pw <KEY_PW>
                     Set the private key password.
    -v, --verify <VERIFY>
                     Set the ca certificate file to use (pem format).
    -P, --print  <PRINT>
                     Print request or response.
"#
)]
struct Args {
    #[arg(index = 1, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(index = 2, required = false, help = "Allow set raw request body.")]
    body: Option<String>,
    #[arg(short, long, help = "HTTP method to use.")]
    method: Option<String>,
    #[arg(short = 'f', long, help = "Set form field, format: key=value.")]
    field: Option<Vec<String>>,
    #[arg(short = 'H', long, help = "Set header field, format: key=value.")]
    header: Option<Vec<String>>,
    #[arg(
        short = 'b',
        long,
        help = "Set bearer auth token on Authorization header."
    )]
    bearer: Option<String>,
    #[arg(
        short = 'a',
        long,
        help = "Set basic auth on Authorization header, format: username=password, it will be base64 encoded."
    )]
    basic: Option<String>,
    #[arg(short = 's', long, help = "Set the file to save the response body.")]
    save: Option<String>,
    #[arg(short = 'p', long, help = "Set the part of multipart/form-data.")]
    part: Option<Vec<String>>,
    #[arg(short = 'c', long, help = "Set the certificate file to use.")]
    cert: Option<String>,
    #[arg(short = 'k', long, help = "Set the certificate password.")]
    cert_pw: Option<String>,
    #[arg(
        short = 'v',
        long,
        help = "Set the ca certificate file to use (pem format)."
    )]
    verify: Option<String>,
    #[arg(short = 'P', long, help = "Print request or response.")]
    print: Option<String>,
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
    let arg_cert = args.cert;
    let arg_cert_pw = args.cert_pw;
    let arg_print = args.print;
    let arg_verify = args.verify;

    if arg_cert.is_some() && arg_cert_pw.is_some() {
        let cert = arg_cert.unwrap();
        let cert_pw = arg_cert_pw.unwrap();
        client.set_client_cert(Some(ClientCert::new(cert, cert_pw, arg_verify)));
    }

    let mut stdin = stdin();
    if !stdin.is_terminal() {
        let mut stdin_body = String::new();
        let result = stdin.read_to_string(&mut stdin_body);
        if let Err(e) = result {
            return Err(DeboaError::Io {
                message: format!("Failed to read from stdin: {}", e),
            });
        }
        arg_body = Some(stdin_body);
    }

    let mut method = "GET".to_string();
    if let Some(some_method) = arg_method {
        method = some_method.to_uppercase();
    } else if arg_body.is_some() || arg_fields.is_some() || arg_part.is_some() {
        method = "POST".to_string();
    }

    let method = method.parse::<Method>();
    if let Err(e) = method {
        return Err(DeboaError::ProcessResponse {
            message: format!("Invalid HTTP method: {}", e),
        });
    }

    if arg_body.is_some() && arg_fields.is_some() && arg_part.is_some() {
        return Err(DeboaError::ProcessResponse {
            message: "Both body, fields and part are set, you can only use one of them."
                .to_string(),
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

    let http_method = method.unwrap();
    let request = DeboaRequest::to(arg_url.as_ref())?;

    let request = if let Some(header) = arg_header {
        header.iter().fold(request, |request, header| {
            let pairs = header.split_once(':');
            let request = if let Some((key, value)) = pairs {
                let header_name = HeaderName::from_bytes(key.as_bytes());
                if let Err(e) = header_name {
                    eprintln!("Error: {:#}", e);
                    return request;
                }
                request.header(header_name.unwrap(), value)
            } else {
                request
            };
            request
        })
    } else {
        request
    };

    let request = if let Some(body) = arg_body {
        request
            .header(header::CONTENT_LENGTH, &body.len().to_string())
            .text(&body)
    } else if let Some(fields) = arg_fields {
        let mut form = EncodedForm::builder();
        for field in fields {
            let pairs = field.split_once('=');
            if let Some((key, value)) = pairs {
                form.field(key, value);
            }
        }
        request.form(form.into())
    } else if let Some(part) = arg_part {
        let mut form = MultiPartForm::builder();
        for part in part {
            let pairs = part.split_once('=');
            if let Some((key, value)) = pairs {
                form.field(key, value);
            }
        }
        request.form(form.into())
    } else {
        request
    };

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
    let request = request.build()?;

    if let Some(print) = arg_print {
        if print == "req" {
            println!("\n\n{} {}", request.method(), request.url());
            println!("Headers: {:#?}", request.headers());
        }
    }

    let response = client.execute(request).await?;

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

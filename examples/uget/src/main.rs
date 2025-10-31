use clap::Parser;
use colored::*;
use colored_json::prelude::*;
use deboa::{
    cert::ClientCert,
    errors::{DeboaError, IoError},
    form::{DeboaForm, EncodedForm, MultiPartForm},
    request::DeboaRequest,
    Deboa, Result,
};
use futures_util::StreamExt;
use http::{HeaderName, Method};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{
    cmp::min,
    io::{stdin, stdout, IsTerminal, Read, Write},
};
use std::{fs::File, path::Path, sync::Arc};
use url::Url;

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
    -r, --resume <RESUME>
                     Resume download from a previous one.
"#
)]
struct Args {
    #[arg(index = 1, required = true, help = "URL to make the request to.")]
    url: String,
    #[arg(index = 2, required = false, help = "Allow set raw request body.")]
    body: Option<String>,
    #[arg(short, long, required = false, help = "HTTP method to use.")]
    method: Option<String>,
    #[arg(
        short = 'f',
        long,
        required = false,
        help = "Set form field, format: key=value."
    )]
    field: Option<Vec<String>>,
    #[arg(
        short = 'H',
        long,
        required = false,
        help = "Set header field, format: key=value."
    )]
    header: Option<Vec<String>>,
    #[arg(
        short = 'b',
        long,
        required = false,
        help = "Set bearer auth token on Authorization header."
    )]
    bearer: Option<String>,
    #[arg(
        short = 'a',
        long,
        required = false,
        help = "Set basic auth on Authorization header, format: username=password, it will be base64 encoded."
    )]
    basic: Option<String>,
    #[arg(
        short = 's',
        long,
        required = false,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "none",
        help = "Set the file to save the response body."
    )]
    save: Option<String>,
    #[arg(
        short = 'p',
        long,
        required = false,
        help = "Set the part of multipart/form-data."
    )]
    part: Option<Vec<String>>,
    #[arg(
        short = 'c',
        long,
        required = false,
        help = "Set the certificate file to use."
    )]
    cert: Option<String>,
    #[arg(
        short = 'k',
        long,
        required = false,
        help = "Set the certificate password."
    )]
    cert_pw: Option<String>,
    #[arg(
        short = 'v',
        long,
        required = false,
        help = "Set the ca certificate file to use (pem format)."
    )]
    verify: Option<String>,
    #[arg(
        short = 'P',
        long,
        value_parser = ["req", "res", "all", "none"],
        num_args = 0..=1,
        default_missing_value = "none",
        required = false,
        help = "Print request or response."
    )]
    print: Option<String>,
    #[arg(
        short = 'q',
        long,
        required = false,
        num_args = 0..=4,
        require_equals = true,
        default_missing_value = "true",
        help = "Show progress bar."
    )]
    bar: Option<bool>,
    #[arg(
        short = 'r',
        long,
        required = false,
        help = "Resume download from a previous one."
    )]
    resume: Option<bool>,
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
    let mut arg_body = args.body;
    let arg_method = args.method;
    let arg_fields = args.field;
    let arg_header = args.header;
    let arg_bearer_auth = args.bearer;
    let arg_basic_auth = args.basic;
    let arg_part = args.part;
    let arg_cert = args.cert;
    let arg_cert_pw = args.cert_pw;
    let arg_print = args.print;
    let arg_verify = args.verify;
    let arg_save = args.save;
    let arg_bar = args.bar;
    let arg_resume = args.resume;

    if let Some((cert, cert_pw)) = arg_cert.zip(arg_cert_pw) {
        client.set_client_cert(Some(ClientCert::new(cert, cert_pw, arg_verify)));
    }

    let mut stdin = stdin();
    if !stdin.is_terminal() {
        let mut stdin_body = String::new();
        let result = stdin.read_to_string(&mut stdin_body);
        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::Stdin {
                message: format!("Failed to read from stdin: {}", e),
            }));
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

    let url = Url::parse(&arg_url);
    if let Err(e) = url {
        return Err(DeboaError::UrlParse {
            message: e.to_string(),
        });
    }

    let url = url.unwrap();

    let request = DeboaRequest::to(url.clone())?;
    let mut expected_size = 0;
    if arg_resume.is_some() && arg_resume.unwrap() {
        let request = request.method(http::Method::HEAD);
        let response = client.execute(request.build()?).await?;
        let content_length = response.content_length()?;
        expected_size = content_length;
    }

    let http_method = method.unwrap();
    let request = DeboaRequest::to(url.clone())?;
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

    let saved_file_name = get_file_from_url(&url)?;
    let saved_file = Path::new(&saved_file_name);
    let request = if saved_file.exists() {
        let actual_size = saved_file.metadata();
        if let Err(e) = actual_size {
            return Err(DeboaError::Io(IoError::File {
                message: format!("Failed to get file metadata: {}", e),
            }));
        }

        let actual_size = actual_size.unwrap().len();
        if expected_size > actual_size {
            request.header(
                http::header::RANGE,
                format!("bytes={}-{}", actual_size, expected_size).as_str(),
            )
        } else {
            request
        }
    } else {
        request
    };

    let request = if let Some(body) = arg_body {
        request.text(&body)
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
        if password.is_empty() && stdin.is_terminal() {
            let mut password = String::new();
            println!("Enter password: ");
            if stdin.read_line(&mut password).is_ok() {
                request.basic_auth(username, password.trim())
            } else {
                eprintln!("Password not provided, exiting.");
                std::process::exit(1);
            }
        } else {
            request.basic_auth(username, password)
        }
    } else {
        request
    };

    let request = request.method(http_method);
    let request = request.build()?;

    if let Some(print) = arg_print.as_ref() {
        if print == "req" || print == "all" {
            println!(
                "\n\n{} {}",
                request.method().to_string().blue(),
                request.url().to_string().white().bold()
            );
            for (key, value) in request.headers() {
                println!(
                    "{}: {}",
                    key.to_string().cyan(),
                    value.to_str().unwrap().yellow()
                );
            }
        }
    }

    let response = client.execute(request).await?;
    let status = response.status();
    let headers = response.headers();

    if let Some(print) = arg_print.as_ref() {
        if print == "res" || print == "all" {
            println!(
                "\n\n{} {} {}",
                client.protocol().to_string().blue(),
                status.as_str().to_string().white().bold(),
                status
                    .canonical_reason()
                    .unwrap_or("<unknown status code>")
                    .to_string()
                    .white()
                    .bold(),
            );
            for (key, value) in headers.iter() {
                println!(
                    "{}: {}",
                    key.to_string().cyan(),
                    value.to_str().unwrap().yellow()
                );
            }
        }
    }

    let mut downloaded = 0u64;

    let content_type = response.content_type()?;

    let content_length = response.content_length()?;

    let mut pb = if arg_bar.unwrap_or(true) {
        let pb = ProgressBar::new(content_length);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
          .unwrap()
          .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
          .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };

    if let Some(mut file_to_save) = arg_save {
        if file_to_save == "none" {
            file_to_save = get_file_from_url(response.url())?;
        }

        let file = File::create(file_to_save);
        if let Ok(mut file) = file {
            let mut stream = response.stream();
            while let Some(frame) = stream.next().await {
                if let Ok(frame) = frame {
                    let new = min(downloaded + frame.len() as u64, content_length);
                    downloaded = new;
                    if let Some(pb) = &mut pb {
                        pb.set_position(new);
                    }
                    let result = file.write(&frame);
                    if let Err(e) = result {
                        return Err(DeboaError::Io(IoError::File {
                            message: format!("Failed to write to file: {}", e),
                        }));
                    }
                }
            }
            let result = file.flush();
            if let Err(e) = result {
                return Err(DeboaError::Io(IoError::File {
                    message: format!("Failed to flush file: {}", e),
                }));
            }
        }
    } else {
        let mut stdout = stdout();
        if content_length < 20000 {
            let is_json = content_type.to_lowercase().contains("application/json");
            let content = response.text().await?;
            if stdout.is_terminal() {
                if is_json {
                    let content = content.to_colored_json(ColorMode::On);
                    if let Ok(content) = content {
                        println!("\n{}", content);
                    } else {
                        eprintln!("Failed to convert to colored JSON");
                    }
                } else {
                    println!("\n{}", content);
                }
            } else {
                println!("\n{}", content);
            }
        } else {
            let mut stream = response.stream();
            while let Some(frame) = stream.next().await {
                if let Ok(frame) = frame {
                    let new = min(downloaded + frame.len() as u64, content_length);
                    downloaded = new;
                    if !stdout.is_terminal() {
                        if let Some(pb) = &mut pb {
                            pb.set_position(new);
                        }
                    }
                    let result = stdout.write(&frame);
                    if let Err(e) = result {
                        return Err(DeboaError::Io(IoError::Stdout {
                            message: format!("Failed to write to stdout: {}", e),
                        }));
                    }
                }
            }
        }

        let result = stdout.flush();
        if let Err(e) = result {
            return Err(DeboaError::Io(IoError::Stdout {
                message: format!("Failed to flush stdout: {}", e),
            }));
        }
    }

    Ok(())
}

fn get_file_from_url(url: &url::Url) -> Result<String> {
    Ok(url.path().split('/').next_back().unwrap().to_string())
}

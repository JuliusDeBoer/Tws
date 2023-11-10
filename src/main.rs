mod file;
mod log;
mod net;
mod page;

use clap::Parser;
use file::FsEntityStatus;
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use std::{convert::Infallible, net::SocketAddr, process};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    help_template = "\n{name} v{version}\n\n{about}\n\n{usage-heading}: {usage}\n\n{all-args}\n\nAuthor:\n{author}\n"
)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Disables all output")]
    quiet: bool,
    #[arg(
        short,
        long,
        default_value = "[::1]:4000",
        help = "What ip address to use"
    )]
    address: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut res = Response::new(Body::empty());

    net::set_default_headers(res.headers_mut());

    let file_status = file::get_fs_entity_status(req.uri().path());

    let path: String = if file_status == FsEntityStatus::IsDir {
        net::parse_url(format!(".{}/index.html", req.uri().path()))
    } else {
        net::parse_url(format!(".{}", req.uri().path()))
    };

    match req.method() {
        &Method::GET => match page::from_file(&path) {
            Ok(b) => {
                *res.status_mut() = StatusCode::OK;
                *res.body_mut() = b;
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_str(file::get_mine_type(path).as_str()).unwrap(),
                );
            }
            Err(e) => {
                *res.status_mut() = e;
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("text/html"),
                );
            }
        },
        &Method::HEAD => {
            if file::get_fs_entity_status(&path) == FsEntityStatus::NotFound {
                *res.status_mut() = StatusCode::NOT_FOUND;
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("text/html"),
                );
            } else {
                *res.status_mut() = StatusCode::OK;
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_str(file::get_mine_type(path).as_str()).unwrap(),
                );
            }
        }
        &Method::OPTIONS => {
            *res.status_mut() = StatusCode::OK;
            res.headers_mut().insert(
                header::ALLOW,
                header::HeaderValue::from_static("GET, HEAD, OPTIONS, TRACE"),
            );
        }
        &Method::TRACE => {
            *res.status_mut() = StatusCode::OK;
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("message/http"),
            );
            *res.body_mut() = page::gen_trace_body(&req);
        }
        &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH | &Method::CONNECT => {
            if file::get_fs_entity_status(&path) == FsEntityStatus::NotFound {
                *res.status_mut() = StatusCode::NOT_FOUND;
            } else {
                *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
                res.headers_mut().insert(
                    header::ALLOW,
                    header::HeaderValue::from_static("GET, HEAD, OPTIONS, TRACE"),
                );
            }
        }
        _ => {
            *res.status_mut() = StatusCode::NOT_IMPLEMENTED;
        }
    }

    log::log_request(&req, &res);

    // Remove this. Maybe?
    if !StatusCode::is_success(&res.status()) {
        *res.body_mut() = page::gen_err_page(res.status());
    }

    Ok(res)
}

async fn shutdown_signal() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        log::warn(format!("Could not prepare graceful shutdown: {}", e));
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    log::set_quiet(args.quiet);

    let addr: SocketAddr = match args.address.parse() {
        Ok(a) => a,
        Err(e) => {
            log::error(format!("Could not bind to address: {}", e));
            process::exit(1);
        }
    };

    log::print_header(&format!("http://{}", args.address));

    let service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    if let Err(e) = Server::try_bind(&addr) {
        log::error(e);
        process::exit(1);
    }

    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        log::error(e);
    }

    if !log::is_quiet() {
        println!("\rBye!");
    }
}

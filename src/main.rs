mod file;
mod log;
mod net;
mod page;
mod request_handlers;

use clap::Parser;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use request_handlers::*;
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

    match req.method() {
        &Method::GET => handle_get(&req, &mut res),
        &Method::HEAD => handle_head(&req, &mut res),
        &Method::OPTIONS => handle_options(&mut res),
        &Method::TRACE => handle_trace(&req, &mut res),
        &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH | &Method::CONNECT => {
            handle_bad_method(&mut res)
        }
        _ => handle_invalid_method(&mut res),
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

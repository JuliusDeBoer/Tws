use crate::log;
use hyper::{Body, Request, StatusCode};
use std::{
    fs,
    io::ErrorKind::{NotFound, PermissionDenied},
};

pub fn from_file(path: &str) -> Result<Body, StatusCode> {
    match fs::read(path) {
        Ok(v) => Ok(Body::from(v)),
        Err(e) => match e.kind() {
            NotFound => Err(StatusCode::NOT_FOUND),
            PermissionDenied => Err(StatusCode::FORBIDDEN),
            _ => {
                log::error(e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}

pub fn gen_err_page(status_code: StatusCode) -> Body {
    Body::from(format!(
        "<!DOCTYPE html><body><h1>{}</h1><h2>{}</h2><body>",
        status_code.as_str(),
        status_code.canonical_reason().unwrap_or("?") // Ed!
    ))
}

pub fn gen_trace_body(req: &Request<Body>) -> Body {
    let mut body = format!("{} {} {:#?}", req.method(), req.uri(), req.version());

    for header in req.headers() {
        body.push_str(
            format!(
                "\n{}: {}",
                header.0.as_str(),
                header.1.to_str().unwrap_or("[BINARY DATA]")
            )
            .as_str(),
        );
    }

    Body::from(body)
}

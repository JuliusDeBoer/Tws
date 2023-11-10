use crate::{file, log};
use hyper::{Body, Request, StatusCode};
use std::fs;

pub fn from_file(path: &str) -> Result<Body, StatusCode> {
    if file::get_fs_entity_status(path) != file::FsEntityStatus::IsFile {
        return Err(StatusCode::NOT_FOUND);
    }

    match fs::read(path) {
        Ok(v) => Ok(Body::from(v)),
        Err(e) => {
            log::error(e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn gen_err_page(status_code: StatusCode) -> Body {
    Body::from(format!(
        " <!DOCTYPE html><body><h1>{}</h1><h2>{}</h2><body>",
        status_code.as_str(),
        status_code.canonical_reason().unwrap_or("?")
    ))
}

pub fn gen_trace_body(req: &Request<Body>) -> Body {
    let mut body = format!("{} {} {:#?}", req.method(), req.uri(), req.version());

    for header in req.headers() {
        match header.1.to_str() {
            Ok(value) => body.push_str(format!("\n{}: {}", header.0.as_str(), value).as_str()),
            Err(..) => {
                body.push_str(format!("\n{}: {}", header.0.as_str(), "[BINARY DATA]").as_str())
            }
        }
    }

    Body::from(body)
}

use crate::{file, file::FsEntityStatus, net, page};
use hyper::{header, Body, Request, Response, StatusCode};

pub fn handle_get(req: &Request<Body>, res: &mut Response<Body>) {
    let file_status = file::get_fs_entity_status(req.uri().path());

    let path: String = if file_status == FsEntityStatus::IsDir {
        net::parse_url(format!(".{}/index.html", req.uri().path()))
    } else {
        net::parse_url(format!(".{}", req.uri().path()))
    };

    match page::from_file(&path) {
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
    }
}

pub fn handle_head(req: &Request<Body>, res: &mut Response<Body>) {
    let entity_status = file::get_fs_entity_status(req.uri().path());
    if entity_status == FsEntityStatus::NotFound {
        *res.status_mut() = StatusCode::NOT_FOUND;
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/html"),
        );
    } else {
        *res.status_mut() = StatusCode::OK;
        if entity_status == FsEntityStatus::IsDir {
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/html"),
            );
        } else {
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_str(
                    file::get_mine_type(String::from(req.uri().path())).as_str(),
                )
                .unwrap(),
            );
        }
    }
}

pub fn handle_options(res: &mut Response<Body>) {
    *res.status_mut() = StatusCode::OK;
    res.headers_mut().insert(
        header::ALLOW,
        header::HeaderValue::from_static("GET, HEAD, OPTIONS, TRACE"),
    );
}

pub fn handle_trace(req: &Request<Body>, res: &mut Response<Body>) {
    *res.status_mut() = StatusCode::OK;
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("message/http"),
    );
    *res.body_mut() = page::gen_trace_body(req);
}

pub fn handle_bad_method(res: &mut Response<Body>) {
    *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
    res.headers_mut().insert(
        header::ALLOW,
        header::HeaderValue::from_static("GET, HEAD, OPTIONS, TRACE"),
    );
}

pub fn handle_invalid_method(res: &mut Response<Body>) {
    *res.status_mut() = StatusCode::NOT_IMPLEMENTED;
}

#[cfg(test)]
mod tests {
    use crate::request_handlers::*;
    use hyper::Uri;

    #[test]
    fn load_file() {
        let mut req = Request::new(Body::default());
        let mut res = Response::new(Body::default());

        *req.uri_mut() = Uri::from_static("http://[::1]:4000/Cargo.toml");

        handle_get(&req, &mut res);

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.headers()[header::CONTENT_TYPE], "text/x-toml");
    }

    #[test]
    fn load_dir() {
        let mut req = Request::new(Body::default());
        let mut res = Response::new(Body::default());

        *req.uri_mut() = Uri::from_static("http://[::1]:4000/test");

        handle_get(&req, &mut res);

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    fn load_nothing() {
        let mut req = Request::new(Body::default());
        let mut res = Response::new(Body::default());

        *req.uri_mut() = Uri::from_static("http://[::1]:4000/IDONTEXIST");

        handle_get(&req, &mut res);

        assert_eq!(res.status().as_u16() / 100, 4);
    }
}

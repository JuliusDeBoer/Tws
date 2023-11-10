use chrono::Local;
use colored::Colorize;
use hyper::{Body, Method, Request, Response};
use std::sync::Mutex;

static QUIET: Mutex<bool> = Mutex::new(false);

pub fn set_quiet(quiet: bool) {
    *QUIET.lock().unwrap() = quiet;
}

pub fn is_quiet() -> bool {
    *QUIET.lock().unwrap()
}

pub fn print_header(url: &String) {
    if is_quiet() {
        return;
    }

    println!();
    println!("  ╭─TWS");
    println!("  │ Welcome back {}!", whoami::realname().blue().bold());
    println!("  │ Version: {}", env!("CARGO_PKG_VERSION").green().bold());
    println!("  │ Host: {}", whoami::devicename().red().bold());
    println!("  │ Address: {}", url.yellow().bold());
    println!("  ╰────────\n");
}

pub fn warn<T: std::fmt::Display>(msg: T) {
    if is_quiet() {
        return;
    }

    let time = Local::now().format("%H:%M:%S").to_string().dimmed();

    println!(" {} {} {}", time, " WARN ".bold().black().on_yellow(), msg);
}

pub fn error<T: std::fmt::Display>(msg: T) {
    if is_quiet() {
        return;
    }

    let time = Local::now().format("%H:%M:%S").to_string().dimmed();

    println!(" {} {} {}", time, " ERROR ".bold().black().on_red(), msg);
}

pub fn log_request(req: &Request<Body>, res: &Response<Body>) {
    if is_quiet() {
        return;
    }

    let time = Local::now().format("%H:%M:%S").to_string().dimmed();

    let method = match *req.method() {
        Method::GET => " GET ".on_green(),
        Method::HEAD => " HEAD ".on_green(),
        Method::POST => " POST ".on_magenta(),
        Method::PUT => " PUT ".on_yellow(),
        Method::DELETE => " DEL ".on_red(),
        Method::CONNECT => " CONN ".on_blue(),
        Method::OPTIONS => " OPT ".on_blue(),
        Method::TRACE => " TRACE ".on_purple(),
        Method::PATCH => " PATCH ".on_yellow(),
        _ => " ??? ".on_white(),
    }
    .bold()
    .black()
    .to_string();

    let code = res.status().as_u16();
    let status_code = match code / 100 {
        1 => format!(" {} ", code).on_purple(),
        2 => format!(" {} ", code).on_green(),
        3 => format!(" {} ", code).on_yellow(),
        4 => format!(" {} ", code).on_red(),
        5 => format!(" {} ", code).on_blue(),
        _ => format!(" {} ", code).on_white(),
    }
    .bold()
    .black()
    .to_string();

    println!(" {}  {}  {:<21}  {}", time, status_code, method, req.uri());
}

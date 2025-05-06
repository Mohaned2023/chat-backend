use axum::http::{HeaderMap, HeaderValue};
use cookie::Cookie;

fn build_header(cookie: String) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(
            &cookie
        ).unwrap()
    );
    return header;
}

pub fn create_auth_header(session: String) -> HeaderMap {
    let cookie = Cookie::build(("session", session))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::days(7)).to_string();
    return build_header(cookie);
}

pub fn empty_auth_header() -> HeaderMap {
    let cookie = Cookie::build(("session", ""))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(0)).to_string();
    return build_header(cookie);
}
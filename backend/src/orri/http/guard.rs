use actix_http::http::{header};
use actix_http::RequestHead;
use actix_web::{guard};
use crate::orri::http as http_helper;


pub fn host_guard(value: &str) -> HostGuard {
    HostGuard(
        header::HeaderValue::from_str(value).unwrap()
    )
}

pub struct HostGuard(header::HeaderValue);


impl guard::Guard for HostGuard {
    fn check(&self, req: &RequestHead) -> bool {
        http_helper::get_host_value(&req.headers) == self.0
    }
}

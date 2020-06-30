use actix_http::http::{header};
use actix_http::RequestHead;
use actix_web::{guard};


pub fn host_guard(value: &str) -> HostGuard {
    HostGuard(
        header::HeaderValue::from_str(value).unwrap()
    )
}

pub struct HostGuard(header::HeaderValue);


impl guard::Guard for HostGuard {
    fn check(&self, req: &RequestHead) -> bool {
        let host = req.headers.get("Host")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(':').next())
            .and_then(|value| header::HeaderValue::from_str(value).ok())
            .unwrap_or_else(|| header::HeaderValue::from_static(""));

        host == self.0
    }
}

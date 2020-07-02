use actix_http::http::{header};
use actix_http::RequestHead;
use actix_web::{guard};
use crate::orri::http::{Host};


pub fn host_guard(value: &str) -> HostGuard {
    HostGuard(
        header::HeaderValue::from_str(value).unwrap()
    )
}

pub struct HostGuard(header::HeaderValue);


impl guard::Guard for HostGuard {
    fn check(&self, req: &RequestHead) -> bool {
        let extensions = req.extensions();
        let host: Option<&Host> = extensions.get();

        host.map(|host| host.0 == self.0)
            .unwrap_or(false)
    }
}

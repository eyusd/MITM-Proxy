use proxyapi::*;
use requests_editor::{body_encoder::decode, header_parser::parser};

#[derive(Clone)]
pub struct RequestInfo {
    request: Option<ProxiedRequest>,
    response: Option<ProxiedResponse>,
}
impl RequestInfo {
    pub fn new(request: Option<ProxiedRequest>, response: Option<ProxiedResponse>)->Self{
        Self {
            request,
            response,
        }
    }
}
impl RequestInfo {
    pub fn show_request(&self) {
        if let Some(r) = &self.request {
        } else {
            println!("No requests");
        }
    }

    pub fn show_response(&self) {
        if let Some(r) = &self.response {
            let parsed = parser(r);
            if parsed.is_of_interest {
                println!("Content - {}", decode(parsed.encoding, r));
            }
        } else {
            println!("No Response");
        }
    }
}

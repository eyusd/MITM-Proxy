use std::io::Read;
use proxyapi::*;
pub mod header_parser;
pub mod body_encoder;

pub fn decompress_body(request: &ProxiedResponse) {
    let mut content_encoding = String::new();
    let mut content_type = String::new();
    for (k, v) in request.headers().iter() {
        if k == "content-encoding" {
            if let Ok(value_str) = v.to_str(){
                content_encoding = value_str.to_string();
            }
        }
        if k == "content-type" {
            if let Ok(value_str) = v.to_str(){
                content_type = value_str.to_string();
            }
        }
    }
    if content_encoding == "gzip" && content_type == "text/html; charset=utf-8" {
        let mut body = String::new();
        let mut decoder = flate2::read::GzDecoder::new(request.body().as_ref());
        decoder.read_to_string(&mut body).unwrap();
        println!("Body - {}", body);
    }
    else if content_encoding == "deflate" && content_type == "text/html; charset=utf-8" {
        let mut body = String::new();
        let mut decoder = flate2::read::DeflateDecoder::new(request.body().as_ref());
        decoder.read_to_string(&mut body).unwrap();
        println!("Body - {}", body);
    }
    else if content_encoding == "br" && content_type == "text/html; charset=utf-8" {
        let mut body = String::new();
        let mut decoder = brotli2::read::BrotliDecoder::new(request.body().as_ref());
        decoder.read_to_string(&mut body).unwrap();
        println!("Body - {}", body);
    }
    else {
        if !content_type.is_empty() {
            println!("Content type - {}", content_type);
            //println!("Body - {}", format!("{:?}",request.body().as_ref()));
        }
    }
}
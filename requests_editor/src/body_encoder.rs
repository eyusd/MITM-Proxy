// takes a vector of http encoding types and a Proxied response, and decode the body
// if the encoding type is supported

use std::io::Read;
use proxyapi::ProxiedResponse;



pub fn decode(encoding: Vec <&str> , response: &ProxiedResponse) -> String {
    let body = response.body().as_ref();
    let mut body_string = String::new();
    match encoding[..] {
        [] => {return std::str::from_utf8(body).unwrap().to_string();}

        [e] => match e {
            "gzip" => {
                match flate2::read::GzDecoder::new(body).read_to_string(&mut body_string) {
                    Ok(_) => {return body_string;}
                    Err(_) => {return String::new();}
                }
            }
            "deflate" => {
                match flate2::read::DeflateDecoder::new(body).read_to_string(&mut body_string) {
                    Ok(_) => {return body_string;}
                    Err(_) => {return String::new();}
                }
            }
            "br" => {
                match brotli2::read::BrotliDecoder::new(body).read_to_string(&mut body_string) {
                    Ok(_) => {return body_string;}
                    Err(_) => {return String::new();}
                }
            }
            "compress" => {
                match flate2::read::ZlibDecoder::new(body).read_to_string(&mut body_string) {
                    Ok(_) => {return body_string;}
                    Err(_) => {return String::new();}
                }
            }
            _ => {return std::str::from_utf8(body).unwrap().to_string();}
        }

        _ => {return String::new()}
    }
}
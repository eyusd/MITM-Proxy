use proxyapi::ProxiedResponse;

pub struct HeaderParser<'l> {
    pub encoding: Vec<&'l str>,
    pub is_of_interest: bool,
}

pub fn parser(r: &ProxiedResponse) -> HeaderParser {
    let mut encoding: Vec<&str> = Vec::new();
    let mut is_of_interest = false;
    if r.status().as_u16() == 200 {
        for (k, v) in r.headers().iter() {
            match k.as_str() {
                "content-encoding" => {
                    if let Ok(value_str) = v.to_str() {
                        for v in value_str.split(',') {
                            encoding.push(v.trim());
                        }
                    }
                }
                "content-type" => {
                    if let Ok(value_str) = v.to_str() {
                        if value_str.contains("text/html") {
                            is_of_interest = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    HeaderParser { encoding: (encoding), is_of_interest: (is_of_interest) }
}


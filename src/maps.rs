use tiny_http;
use reqwest;

#[derive(Debug)]
struct Error {
    msg: String,
}

pub fn request_map(req: &mut tiny_http::Request, ip: String) -> Result<reqwest::RequestBuilder, String> {
    // Map Method
    let method = match reqwest::Method::from_bytes(req.method().as_str().as_bytes()) {
        Ok(m) => m,
        Err(e) => return Err(format!("Error creating method: {}", e)),
    };

    // Map Url
    let url = match reqwest::Url::parse(&format!("{}{}", ip, req.url())[..]) {
        Ok(url) => url,
        Err(e) => return Err(format!("Error creating parsing url: {}", e)),
    };

    // Crete Request
    let client = reqwest::Client::new();
    let mut request = client.request(
        method,
        url,
    );

    // Set Headers
    use reqwest::header::{HeaderName, HeaderValue};
    for header in req.headers().iter() {
        if let (Ok(f), Ok(v)) = (
            HeaderName::from_bytes(header.field.as_str().as_str().as_bytes()),
            HeaderValue::from_str(header.value.as_str())
        ) { request = request.header(f, v); };
    }

    // Map Body
    let mut req_body = String::new();
    if let Ok(_) = req.as_reader().read_to_string(&mut req_body) { request = request.body(req_body); };

    Ok(request)
}

pub fn response_map<T>(res: &reqwest::Response) -> tiny_http::Response<T> {
    unimplemented!();
}

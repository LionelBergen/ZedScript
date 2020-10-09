#[derive(PartialEq)]
#[derive(Debug)]
pub struct HttpError {
    pub error_message : String,
    pub http_response_code : Option<u16>
}

impl HttpError {
}
use crate::util::http_error::HttpError;

pub struct HttpClient {}

/**
 * Class to handle HTTP calls. Basically just a wrapper for an external HTTP client, to make management/changes easier
*/
impl HttpClient {
    pub fn get(url: String) -> Result<String, HttpError> {
        let http_result = Self::http_get_result(url);

        match http_result {
            Ok(result) => {
                if result.status() == 200 {
                    Result::Ok(result.text().unwrap())
                } else if result.status() == 401 {
                    Result::Err(HttpError {
                        error_message: String::from("Unauthorized access to application"),
                        http_response_code: Some(401),
                    })
                } else if result.status() == 403 {
                    Result::Err(HttpError {
                        error_message: String::from(
                            "Forbidden access to application. Check if API key has expired",
                        ),
                        http_response_code: Some(403),
                    })
                } else {
                    Result::Err(HttpError {
                        error_message: String::from("Error in http request"),
                        http_response_code: Some(result.status().as_u16()),
                    })
                }
            }
            Err(error) => Result::Err(HttpError {
                error_message: error.to_string(),
                http_response_code: None,
            }),
        }
    }

    fn http_get_result(url: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        reqwest::blocking::get(&url)
    }
}

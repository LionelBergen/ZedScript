use crate::util::http_error::HttpError;
use std::collections::HashMap;
use reqwest::blocking::Response;
use reqwest::Error;

pub struct HttpClient {}

/**
 * Class to handle HTTP calls. Basically just a wrapper for an external HTTP client, to make management/changes easier
*/
impl HttpClient {
    pub fn get(url: &str) -> Result<String, HttpError> {
        let http_result = Self::http_get_result(url.parse().unwrap());
        Self::request(&*url, http_result)
    }

    pub fn post(url: String, json_paramaters: Option<HashMap<&str, &str>>) -> Result<String, HttpError> {
        let http_result = Self::http_post_result(&url, json_paramaters);
        Self::request(&*url, http_result)
    }

    pub fn put(url: String, json_parameters: Option<HashMap<&str, &str>>) -> Result<String, HttpError> {
        let http_result = Self::http_put_result(&url, json_parameters);
        Self::request(&*url, http_result)
    }

    pub fn request(url: &str, http_result: Result<Response, Error>) -> Result<String, HttpError> {
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

    fn http_put_result(url: &String, json_parameters: Option<HashMap<&str, &str>>) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut request_builder = reqwest::blocking::Client::new().put(url)
            .header("Accept-Charset", "application/x-www-form-urlencoded; charset=UTF-8");

        if json_parameters.is_some() {
            request_builder = request_builder.json(&json_parameters);
        }

        request_builder
            .send()
    }

    fn http_get_result(url: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        reqwest::blocking::get(&url)
    }

    fn http_post_result(url: &String, json_parameters: Option<HashMap<&str, &str>>) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut request_builder = reqwest::blocking::Client::new().post(url)
            .header("Accept-Charset", "application/x-www-form-urlencoded; charset=UTF-8");

        if json_parameters.is_some() {
            request_builder = request_builder.json(&json_parameters);
        }

        request_builder
            .send()
    }
}

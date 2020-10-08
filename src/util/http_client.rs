use reqwest;

pub struct HttpClient {
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct HttpError {
    pub errorMessage : String,
    pub httpResponseCode : Option<u16>
}

impl HttpError {

}

// TODO: this class does not make sense. If it's only used once merge with league_api
/**
 * Class to handle HTTP calls. Basically just a wrapper for an external HTTP client, to make management/changes easier
*/
impl HttpClient {
    pub fn get(url : String) -> Result<String, HttpError> {
        let http_result = Self::http_get_result(url);

        return match http_result {
            Ok(result) => {
                if result.status() == 200 {
                    Result::Ok(String::from(result.text().unwrap()))
                } else if result.status() == 401 {
                    Result::Err(HttpError{ errorMessage: String::from("Unauthorized access to application"),
                        httpResponseCode: Some(401) })
                } else if result.status() == 403 {
                    Result::Err(HttpError{ errorMessage: String::from("Forbidden access to application. Check if API key has expired"),
                        httpResponseCode: Some(403) })
                }else {
                    Result::Err(HttpError{ errorMessage: String::from("Error in http request"),
                        httpResponseCode: Some(result.status().as_u16()) })
                }
            },
            Err(error) => {
                Result::Err(HttpError { errorMessage: error.to_string(), httpResponseCode: None })
            }
        };
    }

    fn http_get_result(url : String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        return reqwest::blocking::get(&url);
    }
}
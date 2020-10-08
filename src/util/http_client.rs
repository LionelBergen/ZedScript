use reqwest;

use super::http_response::HttpResponse;

pub struct HttpClient {
}

pub struct HttpError {
    errorMessage : String,
    httpResponseCode : Option<i32>
}

impl HttpError {

}

/**
 * Class to handle HTTP calls. Basically just a wrapper for an external HTTP client, to make management/changes easier
*/
impl HttpClient {
    pub fn get(url : String) -> Result<String, HttpError> {
        let http_result = Self::http_get_result(url);

        return match http_result {
            Ok(result) => {
                if result.status() == 401 {
                    Result::Err(HttpError{ errorMessage: String::from("Unauthorized access to application"),
                        httpResponseCode: Some(401) })
                } else {
                    println!("+++++++++++++++++++++++");
                    println!("{:#?}", result);
                    println!("{:#?}", result.status());
                    println!("+++++++++++++++++++++++");
                    //let rez : HttpResponse = serde_json::from_str(&zz2).expect("JSON not well formatted");
                    //println!("{:#?}", rez);
                    Result::Ok(String::from("ggnore"))
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
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HttpResponse {
    url: String,
    status: String,
    headers: Vec<String>
}
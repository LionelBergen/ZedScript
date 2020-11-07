use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Observer {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
}
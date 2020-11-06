use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShardStatus {
    pub locales: Vec<String>,
    pub hostname: String,
    pub name: String,
    pub services: Vec<Service>,
    pub slug: String,
    pub region_tag: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Service {
    pub name: String,
    pub slug: String,
    pub status: String,
    pub incidents: Vec<Incident>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Incident {
    pub id: i64,
    pub active: bool,
    pub created_at: String,
    pub updates: Vec<Message>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub id: String,
    pub author: String,
    pub heading: String,
    pub content: String,
    pub severity: String,
    pub created_at: String,
    pub updated_at: String,
    pub translations: Vec<Translation>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Translation {
    pub updated_at: String,
    pub locale: String,
    pub content: String
}
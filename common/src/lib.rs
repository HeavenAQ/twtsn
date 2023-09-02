use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseHeadline {
    pub status: String,
    pub headlines: Vec<(String, String)>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Email {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub service_type: String,
    pub body: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

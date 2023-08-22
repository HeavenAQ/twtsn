use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseHeadline {
    pub status: String,
    pub headlines: Vec<(String, String)>,
}

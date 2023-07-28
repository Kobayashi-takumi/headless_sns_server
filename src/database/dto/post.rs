use super::Dto;
use crate::util::date::now;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Thing,
    pub content: String,
    pub hash_tags: Vec<String>,
    pub mentions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Dto for Post {
    fn table() -> String {
        "posts".to_string()
    }
}

impl Post {
    pub fn new(content: String, hash_tags: Vec<String>, mentions: Vec<String>) -> Self {
        let now = now();
        Self {
            id: Self::id(),
            content,
            hash_tags,
            mentions,
            created_at: now,
            updated_at: now,
        }
    }
}

use mini_markdown::render;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub body: String,
}

impl Post {
    pub fn render(&self) -> String {
        render(&self.body)
    }
}

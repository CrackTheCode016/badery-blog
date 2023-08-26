use serde_json::Result;
use yew::Properties;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq, Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub id: String,
    pub date: String,
    pub author: String,
    pub peek: String,
    pub md_name: String,
}

impl Post {
    pub fn get_post_from_metadata(metadata: String, id: String) -> Result<Post> {
        let metadata: Vec<Post> = serde_json::from_str(&metadata)?;
        let post = metadata
            .iter()
            .find(|p| p.id == id)
            .expect("post not found");
        Ok(post.clone())
    }
}
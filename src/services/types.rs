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
        // console::log_1(&metadata.clone().into());
        let metadata: Vec<Post> = serde_json::from_str(&metadata)?;
        let post = metadata
            .iter()
            .find(|p| p.id == id)
            .expect("post not found");
        Ok(post.clone())
    }
    /// JSON string to Post
    pub fn from_str(str: String) -> Result<Post> {
        serde_json::from_str(&str)
    }
}
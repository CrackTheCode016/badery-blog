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

#[derive(Debug)]
pub enum PostRetrievalError {
    UnableToParseJson,
    PostNotFoundInMetadata

}

impl Post {
    pub fn get_post_from_metadata(metadata: String, id: String) -> Result<Post, PostRetrievalError> {
        let metadata: Vec<Post> = serde_json::from_str(&metadata).map_err(|_| PostRetrievalError::UnableToParseJson)?;
        if let Some(post) = metadata.iter().find(|p| p.id == id) {
           return Ok(post.clone());
        }        
        return Err(PostRetrievalError::PostNotFoundInMetadata);
    }
}
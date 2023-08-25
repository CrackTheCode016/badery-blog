use serde::{Deserialize, Serialize};
use serde_json::Result;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::{use_navigator, Navigator};

use crate::pages::routes::Route;

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

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub posts: Vec<Post>,
}

fn post_to_list_item(post: &Post, navigator: Navigator) -> Html {
    let id: String = post.id.clone();
    let onclick = Callback::from(move |_| {
        let id: String = id.clone();
        navigator.push(&Route::Post { id });
    });

    html! {
     <div class="list-item" {onclick}>
        <div class="list-header">
            <p class="date">{"Tue Aug 22"}</p>
            <h1>{post.title.clone()}</h1>
            <div class="info">
        </div>
    </div>

    <div class="list-body">
        <p>{post.peek.clone()}</p>
        <div class="bar"></div>
        </div>
    </div>
        }
}
/// Creates a formatted list of blog posts from the metadata.json file.
#[function_component(BlogList)]
pub fn blog_partial(props: &BlogListProps) -> Html {
    let navigator = use_navigator().expect("navigator not found");
    // Posts to HTML
    let posts: Vec<VNode> = props
        .posts
        .iter()
        .map(|post| post_to_list_item(post, navigator.clone()))
        .collect();

    html! {
        <div class="list full">
            {posts}
        </div>
    }
}

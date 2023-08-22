use yew::virtual_dom::VNode;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};

use crate::pages::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Post {
    pub title: String,
    pub peek: String,
    pub md: String,
}

impl Post {
    pub fn new(title: &str, post_name: &str, peek: &str) -> Self {
        Post {
            title: title.to_string(),
            peek: peek.to_string(),
            md: post_name.to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub posts: Vec<Post>,
}

fn post_to_list_item(post: &Post, navigator: Navigator) -> Html {
    let md: String = post.md.clone();
    let onclick = Callback::from(move |_| {
        let md: String = md.clone();
        navigator.push(&Route::Post { path: md });
    });

    html! {
    <>
    <div class="list-header">
    <h1>{post.title.clone()}</h1>
    </div>
    <div class="list-body">
        <p>{post.peek.clone()}</p>
        <button {onclick} class="full">{"Read"}</button>
        <div class="bar"></div>
    </div>
    </>
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

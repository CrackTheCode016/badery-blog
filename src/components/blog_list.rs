use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::{use_navigator, Navigator};

use crate::services::types::Post;
use crate::pages::routes::Route;

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
            <p class="date">{post.date.clone()}</p>
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

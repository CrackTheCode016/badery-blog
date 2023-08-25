use wasm_bindgen_futures::spawn_local;
use web_sys::console;
/// The actual page which displays the blog content
use yew::prelude::*;
use yew::suspense::SuspensionResult;
use crate::components::blog_list::{Post, BlogList};
use crate::services::handler::FileHandler;
use crate::services::hooks::use_file;

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    blog: Post,
}

#[function_component(Blog)]
pub fn blog_full() -> Html {

    let md = use_file("/posts/metadata.json".to_string()).unwrap();

    html! {
    <div class="body">
        <h1>{ "Blog" }</h1>
            if md != "not found" {
            <BlogList posts={serde_json::from_str::<Vec<Post>>(&md.clone()).unwrap()} />
            }
    </div>
    }
}

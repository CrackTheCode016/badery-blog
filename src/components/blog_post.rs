use markdown::Options;
use yew::prelude::*;
use yew_router::navigator;
use yew_router::prelude::use_navigator;

use crate::pages::routes::Route;
use crate::services::hooks::use_file;
use crate::services::types::{Post, PostRetrievalError};

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: String,
}

/// The full blog post, loads the markdown content.
#[function_component(BlogPost)]
pub fn post(props: &BlogPostProps) -> HtmlResult {
    let path = format!("/posts/{}.md", props.id.clone());
    let metadata = use_file("/posts/metadata.json".to_string())?;
    let navigator = use_navigator().expect("navigator not available");

    let md: String = use_file(path.clone())?;
    if metadata != "not found" {
        let post = Post::get_post_from_metadata(metadata, props.id.clone());
        // grug brain error checking - simple good!
        if post.is_err() {
            navigator.push(&Route::NotFound);
        } else {
            
            let post = post.expect("NEVER SHOULD BE BAD");
            let raw_html_markdown = markdown::to_html_with_options(&md, &Options::gfm())
                .expect("Unable to parse markdown")
                .lines()
                .skip(6)
                .collect::<String>();

            // todo: should probably be in its own render function, ideally to do things like syntax highlighting or something
            let html = html::Html::from_html_unchecked(raw_html_markdown.into());

            return Ok(html! {
                    <div class="post">
                        <div class="info">
                            <p class="date">{post.date}</p>
                            <p class="author">{post.author}</p>
                        </div>
                            <hr />
                            <h1>{post.title}</h1>
                            <hr />
                         {html}
                    </div>
            });
        }
    }
    return Ok(html! {});
}

use markdown::Options;
use web_sys::console;
use yew::prelude::*;

use crate::services::types::Post;
use crate::services::hooks::use_file;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: String,
}

#[hook]
fn use_test() {
    let state = use_state(|| Some("hi"));
    state.set(Some("hello"));
}

/// The full blog post, loads the markdown content.
#[function_component(BlogPost)]
pub fn post(props: &BlogPostProps) -> HtmlResult {
    let path = format!("/posts/{}.md", props.id.clone());
    let metadata = use_file("/posts/metadata.json".to_string())?;
    let md: String = use_file(path.clone())?;
    if metadata != "not found" {
        let post = Post::get_post_from_metadata(metadata, props.id.clone()).expect("could not parse post");
        let html = html::Html::from_html_unchecked(
            markdown::to_html_with_options(&md, &Options::gfm())
                .expect("Unable to parse markdown")
                .lines()
                .skip(6)
                .collect::<String>()
                .into(),
        );

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

    return Ok(html! {});
}

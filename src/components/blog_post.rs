use markdown::Options;
use yew::suspense::SuspensionResult;
use yew::{platform::spawn_local, prelude::*};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

/// The list of blogs as fetched from the posts directory
const PATH_PREFIX: &str = "/posts";

pub async fn run(path: &String) -> Option<String> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_str(path))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text().unwrap()).await.unwrap();

    text.as_string()
}

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub path: String,
}

// todo: properly use suspension to update inner state, then update component once ready (get rid of loading error)
#[hook]
pub fn use_md(path: String) -> SuspensionResult<String> {
    let state = use_state(|| Some("empty".to_string()));
    let md = {
    let state_handle_clone = state.clone();
    spawn_local(async move {
        let result = run(&path).await.unwrap();
        state_handle_clone.set(Some(result));
    });
    state.as_ref().cloned()
    };
    Ok(md.unwrap())
}


/// The full blog post, loads the markdown content.
#[function_component(BlogPost)]
pub fn post(props: &BlogPostProps) -> Html {
    let md = use_md(format!("{}/{}.md", PATH_PREFIX, props.path.clone())).unwrap();
    let html = html::Html::from_html_unchecked(markdown::to_html_with_options(&md, &Options::gfm()).unwrap().into());
    html! {
        <div class="post">
            <div class="info">
                <p class="date">{"Tue Aug 22"}</p>
                <p class="author">{"Bader Youssef"}</p>
            </div>
                <hr />
                <h1>{"Title Here"}</h1>
                <hr />
                {html}
       </div>
    }
}
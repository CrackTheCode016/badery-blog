use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Nav)]
pub fn navbar() -> Html {
    let navigator = use_navigator().expect("Was unable to get navigator");

    let home_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
          <button {onclick}>{"home"}</button>
        }
    };

    let blog_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Blog));
        html! {
          <button {onclick}>{"blog"}</button>
        }
    };

    html! {
        <nav>
        <div>
          <a class="flex items-center">
            <img src="https://pbs.twimg.com/profile_images/1609252953600188421/d7YNiZNF_400x400.jpg"
              alt="" />
            <p>{"bader y."}</p>
          </a>
          <p>{"lowest effort blog ever."}</p>
          <div class="bar"></div>
          <div>
            {home_btn}
            {blog_btn}
            <a target="_blank" href="https://github.com/CrackTheCode016"> <button>{"github"}</button></a>
            <a target="_blank" href="https://twitter.com/baderyo_o"> <button>{"twitter"}</button></a>
            <a target="_blank" href="https://www.linkedin.com/in/bader-youssef-975914159/"> <button>{"linkedin"}</button></a>
          </div>
        </div>
      </nav>
    }
}

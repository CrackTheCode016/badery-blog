/// not found D:
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {

    let navigator = use_navigator().expect("navigator not found");
    let home_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
          <button {onclick}>{"go back home"}</button>
        }
    };
    html! {
        <div class="body notfound">
            <h1>{ "404, not found :(" }</h1>
            <h1>{ "it seems that you either tried to access a no-no page, or that post doesn't exist. sorry bud!" }</h1>
            {home_btn}
        </div>
    }
}

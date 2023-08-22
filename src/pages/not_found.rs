/// not found D:
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
        <h1>{ "404, not found. sorry partner!" }</h1>
        <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
        </>
    }
}

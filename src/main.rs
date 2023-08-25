mod components;
mod pages;
mod services;

use components::{blog_post::BlogPost, nav::Nav};
use pages::{blog::Blog, home::Home, not_found::NotFound, routes::Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // let dir = std::fs::read_dir("/").unwrap();
    html! {
        <html class="scroll-smooth dark:bg-slate-800">
            <BrowserRouter>
            <Nav />
            <main>
                <Switch<Route> render={switch} />
            </main>
            </BrowserRouter>
        </html>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <Blog /> },
        Route::Post {id} => html! {  <BlogPost {id} />},
        Route::NotFound => html! { <NotFound /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

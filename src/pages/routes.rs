use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/post/:path")]
    Post {path: String},
    #[not_found]
    #[at("/404")]
    NotFound,
}
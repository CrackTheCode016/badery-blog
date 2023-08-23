/// The actual page which displays the blog content
use yew::prelude::*;

use crate::components::blog_list::BlogList;
use crate::components::blog_list::Post;

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    blog: Post,
}

#[function_component(Blog)]
pub fn blog_full() -> Html {
    let posts: Vec<Post> = vec![
        Post::new("Test!", "test", "A simple test post"),
        Post::new("Another test...", "another_test", "INSANNNE MARKDOWN SKILLSZZZ ✅✅✅✅✅✅"),
        Post::new("Real article", "real", "A realistic article."),
    ];
    html! {
    <div class="body">
        <h1>{ "Blog" }</h1>
        <BlogList posts={posts} />
    </div>
    }
}

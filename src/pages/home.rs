/// home!
use yew::prelude::*;

#[function_component(Home)]
pub fn home_page() -> Html {
    html! {
        <div class="columns">
            <h1>{ "welcome to my humble abode." }</h1>
            <p>{ "This is my low effort 'portfolio site' where my work, projects, and ramblings can be found in one place." }</p>
            <div>
                 <p>{ "Made with" }
                    <code> {"yew,"} </code>
                    <code> {"flexbox,"} </code>
                    { "and" }
                    <code> {"love."} </code>
                 </p>
            </div>
            <h2>{ "whoami?" }</h2>
            <p>{
                "my name is bader, and i like to make stuff that matters.
                i am passionate about rust, typescript, systems engineering, embedded development, and blockchain." }
            </p>
            <p>{
                "i also write time to time, of which you may spy on my"}
            <code> {"blog"} </code>
            {"or"}
            <code> {"hackernoon"} </code>
            {"for any of my ramblings.  they mostly have to do with the future of computing, or something like that" }
            </p>
        </div>
    }
}

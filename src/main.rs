use yew::prelude::*;

fn main() {
    println!("Hello, world!");
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"My App"}</h1>
        </>
    }
}

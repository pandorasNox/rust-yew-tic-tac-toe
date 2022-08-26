use yew::{prelude::*, virtual_dom::VNode};

fn main() {
    println!("Hello, world!");
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Tic Tac Toe"}</h1>
            <Grid />
        </>
    }
}

#[function_component(Grid)]
fn grid() -> Html {
    let mut elems: Vec<VNode> = vec![];
    for i in 1..10 {
        let elem = html!{
            <div class="flex justify-center items-center border rounded text-3xl h-24 w-24" >
                {""}
            </div>
        }; 
        elems.push(elem);
    };

    let elems_as_html = elems.into_iter().collect::<Html>();

    html! {
        <div class="flex justify-center">
            <div class="w-80 h-80 grid grid-cols-3 gap-2 border justify-items-center">
                {elems_as_html}
            </div>
        </div>
    }
}



use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    f: Helper,
}

#[function_component(DoF)]
fn doF(props: &Props) -> Html {
    let result = (props.f.0)(0,0);
    html!(result)
}

#[derive(Clone)]
struct Helper(Rc<dyn Fn(usize, usize) -> String>);
impl PartialEq for Helper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// #[function_component(App)]
// fn app() -> Html {
//     let f = Helper(Rc::new(|| "Hello world".to_string()));
//     html! {
//         <DoF f={f} />
//     }
// }

struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {  }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let f = Helper(Rc::new(|_, _| "Hello world".to_string()));
        html! {
            <DoF f={f} />
        }
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();
    yew::start_app::<App>();
}

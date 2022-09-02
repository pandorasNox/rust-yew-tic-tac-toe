use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    f: Helper,
}

#[function_component]
fn DoF(props: &Props) -> Html {
    let result = (props.f.0)();
    html!(result)
}

#[derive(Clone)]
struct Helper(Rc<dyn Fn() -> String>);
impl PartialEq for Helper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[function_component]
fn App() -> Html {
    let f = Helper(Rc::new(|| "Hello world".to_string()));
    html! {
        <DoF f={f} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

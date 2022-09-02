use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    f: Helper,
}

#[function_component(DoF)]
fn doF(props: &Props) -> Html {
    let result = (props.f.0)(0,0);
    html!(<div onclick={result}>{"click me"}</div>)
}

#[derive(Clone)]
struct Helper(Rc<dyn Fn(usize, usize) -> Callback<MouseEvent>>);
impl PartialEq for Helper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

enum Msg { Change(usize, usize) }

struct App {}
impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {  }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let ctxc: &Context<App>  = ctx.clone();
        let f = Helper(Rc::new(|ir, ic| {
            ctxc.link().callback(move |_: MouseEvent| Msg::Change(ir, ic))
        }));

        html! {
            <DoF f={f} />
        }
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();
    yew::start_app::<App>();
}

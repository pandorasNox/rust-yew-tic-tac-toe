use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VNode};
use yew::{function_component, html, Callback};

fn main() { yew::start_app::<App>();}

type TTTGrid = [[i32; 3]; 3];
struct App {grid: TTTGrid}
enum Msg { GridChange(usize, usize) }

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self { Self { grid: [[103; 3]; 3] }}
    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid_change_helper = |ir, ic| {
            ctx.link().callback(move |_: MouseEvent| Msg::GridChange(ir, ic))
        };
        html! { <Grid grid={self.grid} /* onclick_helper={grid_change_helper} */ /> }
    }
}

#[derive(Properties, PartialEq)]
struct GridProps
{
    grid: TTTGrid,
    /* onclick_helper: fn(usize, usize) -> Callback<MouseEvent>, */
}

#[function_component(Grid)]
fn grid(props: &GridProps) -> Html {
  let mut elems: Vec<VNode> = vec![];

  for i_row in 0..props.grid.len() {
      let row = props.grid[i_row];
      for i_col in 0..row.len() {
        elems.push(html!{<div /*onlick={props.onclick_helper(i_row, i_col)}*/>{"..."}</div>})
      }
  }
  let elems_as_html = elems.into_iter().collect::<Html>();

  html! {
    <div>{elems_as_html}</div>
  }
}

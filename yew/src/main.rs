use std::rc::Rc;

use yew::{function_component, html, Callback};
use yew::{prelude::*, virtual_dom::VNode};

fn main() {
    println!("Hello, world!");
    yew::start_app::<App>();
}

struct Game {
    grid: TTTGrid,
}

impl Game {
    fn new() -> Game {
        Game {
            grid: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ],
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Piece {
    X,
    O,
}

type TTTGrid = [[Option<Piece>; 3]; 3];

struct App {
    game: Game,
}

enum Msg {
    Reset,
    GridChange(usize, usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            game: Game::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.game = Game::new();
                true
            }
            Msg::GridChange(i_row, i_col) => {
                match self.game.grid[i_row][i_col] {
                    Some(Piece::X) => {
                        self.game.grid[i_row][i_col] = Some(Piece::O);
                    }
                    Some(Piece::O) => {
                        self.game.grid[i_row][i_col] = None;
                    }
                    None => {
                        self.game.grid[i_row][i_col] = Some(Piece::X);
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let ctx_x = ctx.clone();
        let cb = |_| Msg::Reset;
        let reset = ctx.link().callback(cb);

        let grid_change_helper = |ir, ic| {
            ctx.link().callback(move |_: MouseEvent| Msg::GridChange(ir, ic))
        };
        // let grid_change_helper = |ir, ic| {
        //     Rc::new(
        //         ctx.link().callback(move |_: MouseEvent| Msg::GridChange(ir, ic))
        //     )
        // };

        html! {
            <>
                <h1>{"Tic Tac Toe"}</h1>
                <Grid<_> grid={self.game.grid} onclick_helper={grid_change_helper} />
                <div class="flex justify-center">
                    <button onclick={reset} class="border rounded p-4">{"Reset"}</button>
                </div>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
struct GridProps<F>
where
    F: Fn (usize, usize) -> Callback<MouseEvent> + std::cmp::PartialEq,
{
    grid: TTTGrid,
    onclick_helper: F,
    // onclick_helper: fn(usize, usize) -> Callback<MouseEvent>,
    // onclick_helper: Rc<dyn Fn(usize, usize) -> Callback<MouseEvent>>,
}

#[function_component(Grid)]
fn grid<F>(props: &GridProps<F>) -> Html
where
    F: Fn (usize, usize) -> Callback<MouseEvent> + std::cmp::PartialEq,
{
    let mut elems: Vec<VNode> = vec![];

    for i_row in 0..props.grid.len() {
        let row = props.grid[i_row];
        for i_col in 0..row.len() {
            let onclick: Callback<MouseEvent>= {
                Callback::from(move |ev: MouseEvent| {
                    ev.prevent_default();
                    // article_favorite.run();
                })
            };

            let column_cell = row[i_col];
            match column_cell {
                Some(Piece::X) => {
                    elems.push(html! {
                        <div onclick={onclick} class="flex justify-center items-center border rounded text-3xl h-24 w-24" >
                            {"✕"}
                        </div>
                    });
                }
                Some(Piece::O) => {
                    elems.push(html! {
                        <div onclick={onclick} class="flex justify-center items-center border rounded text-3xl h-24 w-24" >
                            <span class="font-extrabold">{"◯"}</span>
                        </div>
                    });
                }
                None => {
                    elems.push(html! {
                        <div onclick={onclick} class="flex justify-center items-center border rounded text-3xl h-24 w-24" >
                            {""}
                        </div>
                    });
                }
            }
        }
    }

    let elems_as_html = elems.into_iter().collect::<Html>();

    html! {
        <div class="flex justify-center">
            <div class="w-80 h-80 grid grid-cols-3 gap-2 border justify-items-center">
                {elems_as_html}
            </div>
        </div>
    }
}

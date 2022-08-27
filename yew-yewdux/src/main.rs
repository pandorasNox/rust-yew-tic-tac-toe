use std::rc::Rc;

use yew::{function_component, html, prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

fn main() {
    println!("Hello, world!");
    yew::start_app::<App>();
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Piece {
    #[default]
    X,
    O,
}

type TTTGrid = [[Option<Piece>; 3]; 3];

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
struct State {
    grid: TTTGrid,
    current_player: Piece,
}

impl State {
    fn new() -> State {
        State {
            grid: [[None, None, None], [None, None, None], [None, None, None]],
            ..Default::default()
        }
    }

    fn next(&mut self) {
        match self.current_player {
            Piece::X => self.current_player = Piece::O,
            Piece::O => self.current_player = Piece::X,
        }
    }
}

enum Msg {
    Reset,
    PiecePlacement(usize, usize),
}

impl Reducer<State> for Msg {
    fn apply(&self, mut state: Rc<State>) -> Rc<State> {
        let game_state = Rc::make_mut(&mut state);

        match self {
            Msg::Reset => {
                return Rc::new(State::new());
            }
            Msg::PiecePlacement(i_row, i_col) => {
                match game_state.grid[*i_row][*i_col] {
                    None => {
                        game_state.grid[*i_row][*i_col] = Some(game_state.current_player);
                        game_state.next();
                        return Rc::new(game_state.clone());
                    }
                    _ => {}
                }

                return state;
            }
        };
    }
}

#[function_component(App)]
fn app() -> Html {
    let (_, dispatch) = use_store::<State>();
    let reset = dispatch.apply_callback(move |_| Msg::Reset);

    html! {
        <>
            <h1>{"Tic Tac Toe"}</h1>
            <Grid />
            <div class="flex justify-center">
                <button onclick={reset} class="border rounded p-4">{"Reset"}</button>
            </div>
        </>
    }
}

#[function_component(Grid)]
fn grid() -> Html {
    let (game, dispatch) = use_store::<State>();

    let mut elems: Vec<VNode> = vec![];
    for i_row in 0..game.grid.len() {
        let row = game.grid[i_row];
        for i_col in 0..row.len() {
            let onclick = dispatch.apply_callback(move |_| Msg::PiecePlacement(i_row, i_col));

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

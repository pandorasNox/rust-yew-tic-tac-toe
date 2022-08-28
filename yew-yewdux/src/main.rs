use std::{fmt, rc::Rc};

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
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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

    fn has_won(&self) -> (bool, Option<Piece>) {
        let grid = self.grid;
        let mut x_count: u8 = 0;
        let mut o_count: u8 = 0;

        fn validate(x_count: u8, o_count: u8) -> (bool, Option<Piece>) {
            match (x_count, o_count) {
                (3, _) => {
                    return (true, Some(Piece::X));
                }
                (_, 3) => {
                    return (true, Some(Piece::O));
                }
                _ => {
                    return (false, None);
                }
            }
        }

        //check rows
        for i_row in 0..grid.len() {
            for i_col in 0..grid[i_row].len() {
                let elem = grid[i_row][i_col];
                match elem {
                    None => {}
                    Some(Piece::X) => x_count += 1,
                    Some(Piece::O) => o_count += 1,
                }
            }

            let (has_won, winner) = validate(x_count, o_count);
            if has_won { return (has_won, winner);}

            //reset counters
            x_count = 0;
            o_count = 0;
        }

        //check columns
        for i_col in 0..grid[0].len() {
            for i_row in 0..grid[i_col].len() {
                let elem = grid[i_row][i_col];
                match elem {
                    None => {}
                    Some(Piece::X) => x_count += 1,
                    Some(Piece::O) => o_count += 1,
                }
            }

            let (has_won, winner) = validate(x_count, o_count);
            if has_won { return (has_won, winner);}

            //reset counters
            x_count = 0;
            o_count = 0;
        }

        //check diagonals top_left_to_bottom_right
        for i_row in 0..grid.len() {
            let elem = grid[i_row][i_row];
            match elem {
                None => {}
                Some(Piece::X) => x_count += 1,
                Some(Piece::O) => o_count += 1,
            }
        }

        let (has_won, winner) = validate(x_count, o_count);
        if has_won { return (has_won, winner);}

        //reset counters
        x_count = 0;
        o_count = 0;

        //check diagonals top_right_to_bottom_left
        for i_row in 0..grid.len() {
            let last_i = grid.len() - 1;
            let elem = grid[i_row][last_i - i_row];
            match elem {
                None => {}
                Some(Piece::X) => x_count += 1,
                Some(Piece::O) => o_count += 1,
            }
        }

        let (has_won, winner) = validate(x_count, o_count);
        if has_won { return (has_won, winner);}

        //reset counters
        x_count = 0;
        o_count = 0;

        return (false, None);
    }

    fn is_draw(&self) -> bool {
        let (has_won, _) = self.has_won();

        let mut no_nones = true;
        for i_row in 0..self.grid.len() {
            let row = self.grid[i_row];
            if row.contains(&None) {
                no_nones = false;
                break;
            }
        }

        if  no_nones && !has_won {
            return true;
        }

        false
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
                let (has_won, _) = game_state.has_won();
                if has_won {
                    return state;
                }

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
    let (state, dispatch) = use_store::<State>();
    let reset = dispatch.apply_callback(move |_| Msg::Reset);
    let (has_won, winning_piece) = state.has_won();
    let is_draw = state.is_draw();
    let winner = match winning_piece {
        Some(piece) => piece.to_string(),
        _ => "".to_string(),
    };

    html! {
        <>
            <h1>{"Tic Tac Toe"}</h1>
            <div class="flex justify-center">
                <div>
                    <h2>
                        if is_draw {
                            {"It's a draw..."}
                        }

                        if has_won {
                            {"Winner is: "}{winner}
                        }

                        if !is_draw && !has_won {
                            {"..."}
                        }
                    </h2>

                    <Grid />
                </div>
            </div>
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
        <div class="w-80 h-80 grid grid-cols-3 gap-2 border justify-items-center">
            {elems_as_html}
        </div>
    }
}

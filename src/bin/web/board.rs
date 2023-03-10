use chain_reaction::board::{Board, BoardState};
use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::cells::Cell;

pub enum GameBoardAction {
    MoveAnimation,
    Move(usize, usize),
    Reset(u8),
}

pub struct GameBoardState {
    board: RefCell<Board>,
    error: RefCell<String>,
}

impl Reducible for GameBoardState {
    type Action = GameBoardAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        {
            let mut board = self.board.borrow_mut();
            match action {
                GameBoardAction::MoveAnimation => if !board.next_iteration() {},
                GameBoardAction::Move(r, c) => {
                    let cur_player = board.current_player_id();
                    if let Err(msg) = board.player_move(cur_player, r, c) {
                        *self.error.borrow_mut() = format!("{:?}", msg);
                    } else {
                        self.error.borrow_mut().drain(..);
                    };
                }
                GameBoardAction::Reset(players) => {
                    *board = Board::new(10, 10, players);
                }
            }
        }
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct GameBoardPorps {
    pub players: u8,
}

#[function_component(GameBoard)]
pub fn game_board(GameBoardPorps { players }: &GameBoardPorps) -> Html {
    let navigator = use_navigator().unwrap();
    let back_to_menu = { Callback::from(move |_| navigator.push(&Route::Menu)) };
    let game_board_state = use_reducer(|| GameBoardState {
        board: RefCell::new(Board::new(10, 10, *players)),
        error: RefCell::new(String::new()),
    });
    {
        let b = game_board_state.clone();
        use_effect_with_deps(
            move |p| {
                b.dispatch(GameBoardAction::Reset(*p));
            },
            *players,
        )
    }
    let onclick = {
        let b = game_board_state.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            b.dispatch(GameBoardAction::Move(r, c));
        })
    };
    let player_colors = (0..*players)
        .map(|p| {
            format!(
                r#".player-{p} {{color: hsl({h},50%,50%);}}
                   .player-{p} circle {{ filter: hue-rotate({h}deg);}}
                   .player-{p} button {{ font-size: 1.5rem;}}
                "#,
                p = p,
                h = (p as usize) * 360 / (*players as usize),
            )
        })
        .collect::<String>();

    let reset = {
        let players = *players;
        let b = game_board_state.clone();
        Callback::from(move |_| {
            b.dispatch(GameBoardAction::Reset(players));
        })
    };

    let (game_over, cur_player, cells, error) = {
        let board = game_board_state.board.borrow_mut();
        if !matches!(board.state(), BoardState::Wait | BoardState::GameOver(_)) {
            let b = game_board_state.clone();
            Timeout::new(1_000, move || b.dispatch(GameBoardAction::MoveAnimation)).forget();
        }
        (
            matches!(board.state(), BoardState::GameOver(_)),
            board.current_player_id(),
            board.cells(),
            game_board_state.error.borrow().clone(),
        )
    };
    html! {
        <>
        <style>{player_colors}{r#"
        svg {width: 2rem; height: 2rem}
        html {background-color: LightGray;}
        .app {display: flex;align-items: center;flex-direction: column;}
        .app table {border-collapse: collapse; font-size: 2.5em;}
        .app td {
            border: 1px solid white;
            background-image: linear-gradient(LightSlateGray, Black, LightSlateGray);
        }
        .explosion {color: black}
        @keyframes explode {
          from {
            transform: scale(0.5);
          }
          to {
            opacity: 0.5;
            transform: scale(1.5) rotate(10deg);
          }
        }
        .explosion svg {
           animation:  explode 1.5s ;
        }
        @keyframes dance {
          0% {
              transform-origin: center;
              transform: rotate(-45deg);
          }
          100% {
              transform-origin: center;
              transform: rotate(45deg);
          }
        }
        circle {
           animation:  dance 2s infinite alternate;
        }
        "#}</style>
        <div class={classes!("app")}>
            <h1>{ "Chain Reaction" }</h1>
            <h2 class={classes!(format!("player-{}", cur_player))}>
            {if game_over {"Winner: "} else {"Current Player: "} }{cur_player}{"  "}
            <button onclick={back_to_menu}>{"\u{1F519}"}</button>
            <button onclick={reset}>{"\u{1F504}"}</button>
            </h2>
            <p style="color: darkred;">{if !error.is_empty() {&error} else {""} }<br/></p>
            <table>{
                cells.iter().enumerate().map(
                    |(r, row)| html!{<tr>{
                        row.iter().enumerate().map(
                            |(c, cell)|
                            html!{
                                <Cell
                                    state={cell.clone()}
                                    row={r}
                                    col={c}
                                    onclick={onclick.clone()}
                                />
                            }
                        ).collect::<Html>()
                    }</tr>}
                ).collect::<Html>()
            }</table>
        </div>
        </>
    }
}

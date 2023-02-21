use chain_reaction::board::{Board, BoardState};
use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use crate::cells::Cell;

pub enum GameBoardAction {
    MoveAnimation,
    Move(usize, usize),
    Reset,
}

pub struct GameBoardState {
    board: RefCell<Board>,
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
                    if board.player_move(cur_player, r, c).is_ok() {
                        // TODO: handle no change.
                    };
                }
                _ => (),
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
    let game_board_state = use_reducer(|| GameBoardState {
        board: RefCell::new(Board::new(10, 10, *players)),
    });
    {
        let b = game_board_state.clone();
        use_effect_with_deps(
            move |p| {
                b.board.replace(Board::new(10, 10, *p));
                b.dispatch(GameBoardAction::Reset);
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
        .into_iter()
        .map(|p| {
            format!(
                ".player-{p} {{color: hsl({h},{s}%,{l}%);}}\n.player-{p} circle {{fill: hsl({h},{s}%,{l}%);}}",
                p = p,
                h = (p as usize) * 360 / (*players as usize),
                s = 50,
                l = 50
            )
        })
        .collect::<String>();

    let (game_over, cur_player, cells) = {
        let board = game_board_state.board.borrow_mut();
        if !matches!(board.state(), BoardState::Wait | BoardState::GameOver(_)) {
            let b = game_board_state.clone();
            Timeout::new(1_000, move || b.dispatch(GameBoardAction::MoveAnimation)).forget();
        }
        (
            matches!(board.state(), BoardState::GameOver(_)),
            board.current_player_id(),
            board.cells(),
        )
    };
    html! {
        <>
        <style>{player_colors}{r#"
        .app {display: flex;align-items: center;flex-direction: column;}
        .app table {border-collapse: collapse; font-size: 2.5em;}
        .app td {border: 1px solid white; background-image: linear-gradient(gray, black, gray);}
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
            {if game_over {"Winner: "} else {"Current Player: "} }{cur_player}
            </h2>
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

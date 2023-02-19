use chain_reaction::board::{Board, BoardState};
use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::cells::Cell;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub players: u8,
}

#[function_component(App)]
pub fn app(AppProps { players }: &AppProps) -> Html {
    let players = *players;
    let board = use_mut_ref(|| Board::new(10, 10, players));
    let cells = {
        let b = board.clone();
        use_state_eq(|| b.borrow_mut().cells())
    };
    let cur_player = {
        let b = board.clone();
        use_state_eq(|| b.borrow_mut().current_player_id())
    };
    let game_over = use_state_eq(|| false);
    let onclick = {
        let b = board.clone();
        let cells = cells.clone();
        let cur_player = cur_player.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let b: &mut Board = &mut b.borrow_mut();
            if b.player_move(b.current_player_id(), r, c).is_ok() {
                cells.set(b.cells());
                cur_player.set(b.current_player_id());
            }
        })
    };
    {
        let b = board;
        let cells = cells.clone();
        let cur_player = cur_player.clone();
        let game_over = game_over.clone();
        use_effect(move || {
            let timeout = Timeout::new(2_000, move || {
                let b: &mut Board = &mut b.borrow_mut();
                if b.next_iteration() {
                    cells.set(b.cells());
                } else {
                    cur_player.set(b.current_player_id());
                    if matches!(b.state(), BoardState::GameOver(_)) {
                        game_over.set(true);
                    }
                }
            });
            timeout.forget();
        });
    };
    let player_colors = (0..players)
        .into_iter()
        .map(|p| {
            format!(
                ".player-{p} {{color: hsl({h},{s}%,{l}%);}}\n.player-{p} circle {{fill: hsl({h},{s}%,{l}%);}}",
                p = p,
                h = (p as usize) * 360 / (players as usize),
                s = 50,
                l = 50
            )
        })
        .collect::<String>();
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
            <h2 class={classes!(format!("player-{}", *cur_player))}>
            {if *game_over {"Winner: "} else {"Current Player: "} }{*cur_player}
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

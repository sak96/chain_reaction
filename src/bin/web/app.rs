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
    let onclick = {
        let b = board.clone();
        let cells = cells.clone();
        let cur_player = cur_player.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let b: &mut Board = &mut *b.borrow_mut();
            if b.player_move(b.current_player_id(), r, c).is_ok() {
                cells.set(b.cells());
                cur_player.set(b.current_player_id());
            }
        })
    };
    {
        let b = board.clone();
        let cells = cells.clone();
        let cur_player = cur_player.clone();
        use_effect(move || {
            let timeout = Timeout::new(1_000, move || {
                let b: &mut Board = &mut *b.borrow_mut();
                if b.next_iteration() {
                    cells.set(b.cells());
                } else {
                    cur_player.set(b.current_player_id());
                }
            });
            timeout.forget();
        });
    };
    let game_over = matches!(board.borrow_mut().state(), BoardState::GameOver(_));
    let player_colors = (0..players)
        .into_iter()
        .map(|p| {
            format!(
                ".player-{} {{color: hsl({},{}%,{}%);}}\n",
                p,
                (p as usize) * 360 / (players as usize),
                50,
                50
            )
        })
        .collect::<String>();
    html! {
        <>
        <style>{player_colors}{r#"
        .explosion {color: black}
        "#}</style>
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <h2 class={classes!(format!("player-{}", *cur_player))}>
            {if game_over {"Winner: "} else {"Current Player: "} }{*cur_player}
            </h2>
            <table style="border-collapse: collapse;font-size: 2.5em;">{
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
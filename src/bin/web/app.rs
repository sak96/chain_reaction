use chain_reaction::board::{Board, BoardState};
use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::cells::Cell;
use crate::color::get_hsl_player_color;

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
    let (h, s, l) = get_hsl_player_color(*cur_player, players);
    html! {
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <h2 style={format!("color:hsl({},{}%,{}%);",h,s,l)}>
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
                                    player_count={players}
                                />
                            }
                        ).collect::<Html>()
                    }</tr>}
                ).collect::<Html>()
            }</table>
        </div>
    }
}

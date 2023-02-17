use chain_reaction::board::{Board, BoardState, CellState};
use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub players: u8,
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub state: CellState,
    pub row: usize,
    pub col: usize,
    pub player_count: u8,
    pub onclick: Callback<(usize, usize), ()>,
}

type HSLColor = (usize, usize, usize);

fn get_hsl_player_color(id: u8, total: u8) -> HSLColor {
    ((id as usize) * 360 / (total as usize), 50, 50)
}
const EXPLOSION_HSL: HSLColor = (0, 0, 50);
const EMPTY_HSL: HSLColor = (0, 100, 100);

#[function_component(Cell)]
fn cell(
    CellProps {
        state,
        row,
        col,
        onclick,
        player_count,
    }: &CellProps,
) -> Html {
    let row = *row;
    let col = *col;
    let (content, (h, s, l)) = match state {
        CellState::Explosion => ('X'.to_string(), EXPLOSION_HSL),
        CellState::NonEmpty(owner_id, atoms) => {
            let color = get_hsl_player_color(*owner_id, *player_count);
            (atoms.to_string(), color)
        }
        CellState::Empty => ('0'.to_string(), EMPTY_HSL),
    };
    html! {
        <td style={format!("border: 1px solid black;background-color:hsl({}, {}%, {}%);",h,s,l)}
            onclick={let onclick = onclick.clone(); move |_| { onclick.emit((row,col)) } }>
            {content}
        </td>
    }
}

#[function_component(App)]
fn app(AppProps { players }: &AppProps) -> Html {
    let players = *players;
    let board = use_mut_ref(|| Board::new(10, 10, players));
    let cells = {
        let b = board.clone();
        use_state_eq(|| b.borrow_mut().cells())
    };
    let onclick = {
        let b = board.clone();
        let cells = cells.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let b: &mut Board = &mut *b.borrow_mut();
            if b.player_move(b.current_player_id(), r, c).is_ok() {
                cells.set(b.cells());
            }
        })
    };
    {
        let b = board.clone();
        let cells = cells.clone();
        use_effect(move || {
            let timeout = Timeout::new(1_000, move || {
                let b: &mut Board = &mut *b.borrow_mut();
                if b.next_iteration() {
                    cells.set(b.cells());
                }
            });
            timeout.forget();
        });
    };
    let cur_player = board.borrow_mut().current_player_id();
    let game_over = matches!(board.borrow_mut().state(), BoardState::GameOver(_));
    let (h, s, l) = get_hsl_player_color(cur_player, players);
    html! {
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <h2 style={format!("color:hsl({},{}%,{}%);",h,s,l)}>
            {if game_over {"Winner: "} else {"Current Player: "} }{cur_player}
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

fn main() {
    yew::Renderer::<App>::with_props(AppProps { players: 2 }).render();
}

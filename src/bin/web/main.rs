use chain_reaction::board::{Board, CellState};
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
}

#[function_component(Cell)]
fn cell(
    CellProps {
        state,
        row: _row,
        col: _col,
    }: &CellProps,
) -> Html {
    html! {
        match state {
            CellState::Explosion => "+1".to_string(),
            CellState::NonEmpty(owner_id,atoms) => format!("{}{}", owner_id, atoms),
            CellState::Empty => "\u{00a0}\u{00a0}".to_string(),
        }
    }
}

#[function_component(App)]
fn app(AppProps { players }: &AppProps) -> Html {
    let board = use_mut_ref(|| Board::new(10, 10, *players));
    let cells = {
        let b= board.clone();
        use_state_eq(|| b.borrow_mut().cells())
    };
    let onclick = {
        let b = board.clone();
        let cells = cells.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let b: &mut Board =  &mut *b.borrow_mut();
            if b.player_move(b.current_player_id(), r, c).is_ok() {
                while b.next_iteration() {}
                cells.set(b.cells());
            }
        })
    };
    html! {
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <table style="border-collapse: collapse;font-size: 2.5em;">{
                cells.iter().enumerate().map(
                    |(r, row)| html!{<tr>{
                        row.iter().enumerate().map(
                            |(c, cell)|
                            html!{<td style="border: 1px solid black;" onclick={
                                let movement = onclick.clone();
                                move |_| { movement.emit((r,c)) }
                            }>
                                // TODO: Fix cell clone please!!!
                                <Cell state={cell.clone()} row={r} col={c}/>
                            </td>}
                        ).collect::<Html>()
                    }</tr>}
                ).collect::<Html>()
            }</table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::with_props(AppProps { players: 2}).render();
}

use chain_reaction::board::{Board, CellState};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub board: Board,
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
fn app(AppProps { board }: &AppProps) -> Html {
    html! {
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <table style="border-collapse: collapse;font-size: 2.5em;">{
                board.cells().iter().enumerate().map(
                    |(r, row)| html!{<tr>{
                        row.iter().enumerate().map(
                            |(c, cell)|
                            // TODO: Fix clone please!!!
                            html!{<td style="border: 1px solid black;">
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
    let board = Board::new(10, 10, 2);
    yew::Renderer::<App>::with_props(AppProps { board }).render();
}

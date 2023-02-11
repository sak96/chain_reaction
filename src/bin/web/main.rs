use yew::prelude::*;
use chain_reaction::board::{Board, CellState};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub board: Board,
}


#[function_component(App)]
fn app(props: &AppProps) -> Html {
    html! {
        <div style="display: flex;align-items: center;flex-direction: column;">
            <h1>{ "Chain Reaction" }</h1>
            <table style="border-collapse: collapse;font-size: 2.5em;">{
                props.board.cells().iter().enumerate().map(
                    |(r, row)| html!{<tr>{
                        row.iter().enumerate().map(
                            |(c, cell)| html!{<td style="border: 1px solid black;">{
                                match cell {
                                    CellState::Explosion => "+1".to_string(),
                                    CellState::NonEmpty(owner_id,atoms) => format!("{}{}", owner_id, atoms),
                                    CellState::Empty => "\u{00a0}\u{00a0}".to_string(),
                                }
                            }</td>}
                        ).collect::<Html>()
                    }</tr>}
                ).collect::<Html>()
            }</table>
        </div>
    }
}

fn main() {
    let board = Board::new(10, 10, 2);
    yew::Renderer::<App>::with_props(AppProps{board}).render();
}

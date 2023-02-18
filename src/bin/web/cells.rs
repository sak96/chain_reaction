use chain_reaction::board::CellState;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub state: CellState,
    pub row: usize,
    pub col: usize,
    pub onclick: Callback<(usize, usize), ()>,
}

#[function_component(Cell)]
pub fn cell(
    CellProps {
        state,
        row,
        col,
        onclick,
    }: &CellProps,
) -> Html {
    let row = *row;
    let col = *col;
    let (content, class) = match state {
        CellState::Explosion => ('*'.to_string(), "explosion".to_string()),
        CellState::NonEmpty(owner_id, atoms) => {
            (atoms.to_string(), format!("player-{}", *owner_id))
        }
        CellState::Empty => ('0'.to_string(), "".to_string()),
    };
    html! {
        <td style={"border: 1px solid black;"} class={classes!(class)}
            onclick={let onclick = onclick.clone(); move |_| { onclick.emit((row,col)) } }>
            {content}
        </td>
    }
}

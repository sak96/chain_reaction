use chain_reaction::board::CellState;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub state: CellState,
    pub row: usize,
    pub col: usize,
    pub player_count: u8,
    pub onclick: Callback<(usize, usize), ()>,
}
use crate::color::*;

#[function_component(Cell)]
pub fn cell(
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
        CellState::Explosion => ('*'.to_string(), EXPLOSION_HSL),
        CellState::NonEmpty(owner_id, atoms) => {
            let color = get_hsl_player_color(*owner_id, *player_count);
            (atoms.to_string(), color)
        }
        CellState::Empty => ('0'.to_string(), EMPTY_HSL),
    };
    html! {
        <td style={format!("border: 1px solid black;color:hsl({}, {}%, {}%);",h,s,l)}
            onclick={let onclick = onclick.clone(); move |_| { onclick.emit((row,col)) } }>
            {content}
        </td>
    }
}

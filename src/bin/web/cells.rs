use chain_reaction::board::CellState;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub state: CellState,
    pub row: usize,
    pub col: usize,
    pub onclick: Callback<(usize, usize), ()>,
}

const EXPLOSION: &str = include_str!("assets/explosion.svg");
const CELL_1: &str = include_str!("assets/1_cells.svg");
const CELL_2: &str = include_str!("assets/2_cells.svg");
const CELL_3: &str = include_str!("assets/3_cells.svg");
const EMPTY: &str = include_str!("assets/empty.svg");

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
        CellState::Explosion => (EXPLOSION, "explosion".to_string()),
        CellState::NonEmpty(owner_id, atoms) if *atoms <= 3 => {
            let content = match atoms {
                1 => CELL_1,
                2 => CELL_2,
                3 => CELL_3,
                _ => unreachable!(),
            };
            (content, format!("player-{}", *owner_id))
        }
        _ => (EMPTY, "".to_string()),
    };
    let parsed_html = Html::from_html_unchecked(AttrValue::from(content));
    html! {
        <td class={classes!(class)}
            onclick={let onclick = onclick.clone(); move |_| { onclick.emit((row,col)) } }>
            {parsed_html}
        </td>
    }
}

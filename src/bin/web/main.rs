mod app;
pub mod cells;
pub mod color;

use app::{App, AppProps};

fn main() {
    yew::Renderer::<App>::with_props(AppProps { players: 2 }).render();
}

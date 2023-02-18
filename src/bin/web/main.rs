mod app;
pub mod cells;

use app::{App, AppProps};

fn main() {
    yew::Renderer::<App>::with_props(AppProps { players: 2 }).render();
}

mod app;
pub mod board;
pub mod cells;
pub mod menu;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

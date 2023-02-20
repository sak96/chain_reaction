mod app;
pub mod board;
pub mod cells;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

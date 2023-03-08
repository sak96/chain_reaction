mod app;
pub mod board;
pub mod cells;
pub mod menu;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::Renderer::<App>::new().render();
}

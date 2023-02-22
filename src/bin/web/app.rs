use yew::prelude::*;
use yew_router::prelude::*;

use crate::board::GameBoard;
use crate::menu::Menu;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/:players")]
    App { players: u8 },
    #[not_found]
    #[at("/")]
    Menu,
}

fn switch_route(routes: Route) -> Html {
    match routes {
        Route::App { players } => html! {<GameBoard players={players} />},
        Route::Menu => html! { <Menu /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch_route} />
        </HashRouter>
    }
}

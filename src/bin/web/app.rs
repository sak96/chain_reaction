use yew::prelude::*;
use yew_router::prelude::*;

use crate::board::GameBoard;
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:players")]
    App { players: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_route(routes: Route) -> Html {
    match routes {
        Route::App { players } => html! {<GameBoard players={players} />},
        Route::NotFound => html! {
            <Redirect<Route> to={Route::App {players: 2}}/>
        },
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

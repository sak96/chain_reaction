use crate::app::Route;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let navigator = use_navigator().unwrap();
    let players = use_state(|| 2);
    let onclick = {
        let p = players.clone();
        Callback::from(move |_| navigator.push(&Route::App { players: *p }))
    };
    let players_changed = {
        let players = players.clone();
        Callback::from(move |i: u8| players.set(i))
    };
    html! {
        <>
            <style>{r#"
            .menu {
                padding: 1rem; 
                display: flex;
                align-items: center;
                flex-direction: column;
            }
            .menu * {
                margin: 1rem; 
                padding: 0.5rem; 
            }
            "#}</style>
            <div class={classes!("menu")}>
                <label for="players">{"Players: "}</label>
                <Number max_value=10 min_value=2 inital_value={*players} update={players_changed} />
                <button {onclick}>{"Start Game"}</button>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct NumberProps {
    #[prop_or(None)]
    pub inital_value: Option<u8>,
    #[prop_or(0)]
    pub max_value: u8,
    #[prop_or(u8::MAX)]
    pub min_value: u8,
    pub update: Callback<u8>,
}

#[function_component(Number)]
pub fn number(
    NumberProps {
        max_value,
        min_value,
        inital_value,
        update,
    }: &NumberProps,
) -> Html {
    let input_node_ref = use_node_ref();
    let min = *min_value;
    let max = if min_value > max_value {
        min_value
    } else {
        max_value
    };
    {
        let input_node_ref = input_node_ref.clone();
        let inital_value = *inital_value;
        let min_value = *min_value;
        let update = update.clone();
        use_effect_with_deps(
            move |_| {
                let value = inital_value.unwrap_or(min_value);
                if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                    input.set_value(&value.to_string());
                    update.emit(value);
                }
            },
            (),
        )
    };
    let onchange = {
        let input_node_ref = input_node_ref.clone();
        let update = update.clone();
        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                if let Ok(value) = input.value().parse() {
                    // if value < max && value > min {
                    update.emit(value);
                    // }
                }
            }
        })
    };

    html! {
        <input ref={input_node_ref}
            {onchange}
            max={max.to_string()}
            min={min.to_string()}
            type="number"
        />
    }
}

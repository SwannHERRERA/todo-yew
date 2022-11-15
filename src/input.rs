use yew::prelude::*;

#[function_component(Input)]
pub fn input() -> Html {
    let _state = use_state(String::new);
    html! {
        <>
            <input />
        </>
    }
}

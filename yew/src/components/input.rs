use web_sys::KeyboardEvent;
use yew::{prelude::*, html::Scope};

use crate::{app::App, message::Msg};

#[function_component(Input)]
pub fn input() -> Html {
    let link: Scope<App> = todo!();
    let onkeypress = link.batch_callback(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input: InputElement = e.target_unchecked_into();
            let value = input.value();
            input.set_value("");
            Some(Msg::Add(value))
        } else {
            None
        }
    });
    html! {
        <input
            class="new-todo"
            placeholder="What needs to be done?"
            {onkeypress}
        />
    }
}

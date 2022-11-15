use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Input)]
pub fn input() -> Html {
    let todo = use_state(String::new);
    let onchange = {
        println!("{:?}", &todo);
        let todo = todo.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            todo.set(input.value());
        })
    };
    html! {
        <>
            <input {onchange} value={(*todo).clone()} />
        </>
    }
}

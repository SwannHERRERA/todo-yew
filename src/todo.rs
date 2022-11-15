use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoProps {
    pub id: String,
    pub content: String,
    pub is_completed: bool,
}

#[function_component(Todo)]
pub fn todo(props: &TodoProps) -> Html {
    html! {
        <>
            <div>
                <input checked={props.is_completed} type="checkbox" id={props.id.clone()}/>
                <label for={props.id.clone()}>{ props.content.clone() }</label>
            </div>
        </>
    }
}

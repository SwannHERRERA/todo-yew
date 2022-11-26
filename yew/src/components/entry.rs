use yew::{function_component, html, Classes, Properties};

use crate::message::Msg;
use crate::state::Entry;

#[derive(Debug, Properties, PartialEq)]
pub struct EntryProps {
    idx: usize,
    entry: Entry,
}

#[function_component(Entry)]
pub fn entry(props: &EntryProps) -> Html {
    let idx = todo!();
    let entry = todo!();
    let link  = todo!();
    let mut class = Classes::from("todo");
    if entry.editing {
        class.push(" editing");
    }
    if entry.completed {
        class.push(" completed");
    }
    html! {
        <li {class}>
            <div class="view">
                <input
                    type="checkbox"
                    class="toggle"
                    checked={entry.completed}
                    onclick={link.callback(move |_| Msg::Toggle(idx))}
                />
                <label ondblclick={link.callback(move |_| Msg::ToggleEdit(idx))}>{ &entry.description }</label>
                <button class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
            </div>
            { self.view_entry_edit_input((idx, entry), link) }
        </li>
    }
}

#[function_component(EntryEditInput)]
pub fn entry_edit_input(props: &EntryProps) -> Html {
    let edit = move |input: InputElement| {
        let value = input.value();
        input.set_value("");
        Msg::Edit((idx, value))
    };

    let onblur = link.callback(move |e: FocusEvent| edit(e.target_unchecked_into()));

    let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
        (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
    });

    if entry.editing {
        html! {
            <input
                class="edit"
                type="text"
                ref={focus_ref.clone()}
                value={state.edit_value.clone()}
                onmouseover={link.callback(|_| Msg::Focus)}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}

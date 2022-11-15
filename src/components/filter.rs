use yew::html::Scope;
use yew::prelude::*;
use crate::app::App;
use crate::state::Filter;
use crate::message::Msg;

#[derive(Properties, PartialEq)]
pub struct FilterComponentProps {
    pub subject: Filter,
    pub active_filter: Filter,
}

#[function_component(FilterComponent)]
pub fn filter(props: &FilterComponentProps) -> Html {
    let link: &Scope<App> = use_context().unwrap();
    let subject = props.subject;
    let cls = if props.active_filter == subject {
        "selected"
    } else {
        "not-selected"
    };
    html! {
        <li>
            <a class={cls}
               href={subject.as_href()}
               onclick={link.callback(move |_| Msg::SetFilter(subject))}
            >
                { subject }
            </a>
        </li>
    }
}

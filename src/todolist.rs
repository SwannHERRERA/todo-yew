use yew::prelude::*;

use crate::utils::generate_random_id;
use crate::todo::Todo;
use crate::input::Input;

pub enum TodoListAction {

}

pub struct TodoList {
    todos: Vec<String>,
}

impl Component for TodoList {
    type Message = TodoListAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            todos: vec!["toot".to_owned(), "at".to_owned(), "behind".to_owned()],
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Input/>
                <ul>
                    {
                        self.todos.clone().iter().map(|text| {
                                let props = yew::props!(Todo::Properties {
                                    id: generate_random_id(),
                                    content: text.clone(),
                                    is_completed: false,
                                });
                                html! { <Todo ..props /> }                               
                        }).collect::<Html>()
                    }
                </ul>
            </>
        }
    }
}

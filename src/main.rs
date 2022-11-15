use todolist::TodoList;
mod utils;
mod todo;
mod input;
mod todolist;

fn main() {
    yew::start_app::<TodoList>();
}


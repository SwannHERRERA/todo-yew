mod config;
mod message;
mod components;
mod app;
mod state;

use app::App;


fn main() {
    yew::start_app::<App>();
}


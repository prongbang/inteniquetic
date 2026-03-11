mod app;
mod components;
mod pages;
mod services;
mod state;
mod types;
mod utils;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    });
}

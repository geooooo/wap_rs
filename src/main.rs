mod components;
mod state;
mod web;

use leptos::prelude::*;
use console_error_panic_hook;
use components::App;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { 
        <App />
    })
}
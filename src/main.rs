mod components;
mod state;

use leptos::prelude::*;
use components::App;

fn main() {
    leptos::mount::mount_to_body(|| view! { 
        <App />
    })
}
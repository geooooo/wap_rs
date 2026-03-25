use leptos::prelude::*;

#[component]
pub fn PrevFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_prev">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="124.7 265 15.2 20"><path d="M138 284q-12-6-12-9 0-2 12-9m1 15q-8-4-8-6t8-5"/></svg>
        </button>
    }
}

#[component]
pub fn PlayFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_play">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="157.7 263.2 24.5 23.5"><path d="M158 285v-20q0-2 2-1l11 5 10 5q3 1 0 2l-10 5-11 5q-2 1-2-1"/></svg>
        </button>
        <button class="flat-button flat-button_pause">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="159.5 262.4 15 25"><path d="m162 262 3 3v19q-1 3-3 3t-2-3v-19zm10 0 3 3v19q-1 3-3 3t-2-3v-19z"/></svg>
        </button>
    }
}

#[component]
pub fn NextFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_next">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="194.5 265 15.2 20"><path d="M196 266q13 7 13 9 0 3-13 9m-1-14q8 3 8 5t-8 6"/></svg>
        </button>
    }
}

#[component]
pub fn ListFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_list">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="299 268.5 18 13"><path d=" M 299 270 C 299 269.172 299.672 268.5 300.5 268.5 C 301.328 268.5 302 269.172 302 270 C 302 270.828 301.328 271.5 300.5 271.5 C 299.672 271.5 299 270.828 299 270 Z  M 299 275 C 299 274.172 299.672 273.5 300.5 273.5 C 301.328 273.5 302 274.172 302 275 C 302 275.828 301.328 276.5 300.5 276.5 C 299.672 276.5 299 275.828 299 275 Z  M 299 280 C 299 279.172 299.672 278.5 300.5 278.5 C 301.328 278.5 302 279.172 302 280 C 302 280.828 301.328 281.5 300.5 281.5 C 299.672 281.5 299 280.828 299 280 Z  M 304 270 L 317 270M 304 280 L 317 280 M 304 275 L 317 275"></path></svg>
        </button>
    }
}

#[component]
pub fn RandomFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_random">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="261.9 267 18.7 15"><path d="m264 268 7 9 7 3m-15 1 7-9 7-3m-1 8 4 4h-5m-2-13 5 1-3 3"/></svg>
        </button>
    }
}

#[component]
pub fn LoopFlatButton() -> impl IntoView {
    view! {
        <button class="flat-button flat-button_loop">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="223.9 265 21 19.5"><path d="m237 271-1-5h5m-4 1q7 2 7 8c0 5-6 9-10 9q-7 0-9-9 0-7 8-8"/></svg>
        </button>
    }
}
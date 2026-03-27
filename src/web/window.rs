use leptos::web_sys::HtmlElement;
use leptos::wasm_bindgen::JsCast;
use leptos_use::use_event_listener;
use leptos::leptos_dom::helpers::window;
use leptos::ev;

pub fn init_global_key_event_handlers(
    on_random: impl Fn() + 'static,
    on_loop: impl Fn() + 'static,
    on_list: impl Fn() + 'static,
    on_play: impl Fn() + 'static,
    on_volume: impl Fn(bool) + 'static,
    on_next: impl Fn() + 'static,
    on_prev: impl Fn() + 'static,
    on_remove: impl Fn() + 'static,
    on_deselect_all: impl Fn() + 'static,
    on_select_all: impl Fn() + 'static,
) {
    let window = window();
    let document = window.document().unwrap();

    let _ = use_event_listener(window.clone(), ev::keydown, move |_| {
        let focused_element = document
            .active_element()
            .unwrap()
            .dyn_into::<HtmlElement>();
        if let Ok(focused_element) = focused_element {
            focused_element.blur().unwrap();
        }
    });

    let _ = use_event_listener(window.clone(), ev::keyup, move |event| {
        match event.key().as_str() {
            "r" | "R" | "к" | "К" => on_random(),
            "l" | "L" | "д" | "Д" => on_loop(),
            "Enter" => on_list(),
            " " | "p" | "P" | "з" | "З" => on_play(),
            "+" => on_volume(true),
            "-" => on_volume(false),
            "ArrowUp" | "ArrowLeft" => on_prev(),
            "ArrowDown" | "ArrowRight" => on_next(),
            "Backspace" => on_remove(),
            "Escape" => on_deselect_all(),
            _ => (),
        }

        event.stop_propagation();
        event.prevent_default();
    });

    let _ = use_event_listener(window.clone(), ev::keydown, move |event| {
        match event.key().as_str() {
            "a" | "A" | "ф" | "Ф" if event.meta_key() => on_select_all(),
            _ => (),
        }

        event.stop_propagation();
        event.prevent_default();
    });
}
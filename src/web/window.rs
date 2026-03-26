use leptos::web_sys::HtmlElement;
use leptos::wasm_bindgen::JsCast;
use leptos_use::use_event_listener;
use leptos::leptos_dom::helpers::window;
use leptos::ev;

pub fn init_global_key_event_handlers(
    on_random_key: impl Fn() + 'static,
    on_loop_key: impl Fn() + 'static,
    on_list_key: impl Fn() + 'static,
    on_play_key: impl Fn() + 'static,
    on_volume_key: impl Fn(bool) + 'static,
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
            "r" | "R" | "к" | "К" => on_random_key(),
            "l" | "L" | "д" | "Д" => on_loop_key(),
            "Enter" => on_list_key(),
            " " | "p" | "P" | "з" | "З" => on_play_key(),
            "+" => on_volume_key(true),
            "-" => on_volume_key(false),
            _ => return,
        }

        event.stop_propagation();
        event.prevent_default();
    });
}
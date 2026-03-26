use leptos::ev;
use leptos::prelude::*;
use leptos::html;
use leptos_use::{use_event_listener, use_throttle_fn_with_arg};
use std::rc::Rc;

#[component]
pub fn MoveLineTime(
    duration: Signal<u32>,
    current_time: Signal<u32>,
) -> impl IntoView {
    let bar_ref: NodeRef<html::Div> = NodeRef::new();

    Effect::new(move |_| {
        let bar_element = bar_ref.get().unwrap();
        let bar_width = f64::trunc(
            (current_time.get() as f64 / duration.get() as f64) * 100.0
            
        ) as u32;

        bar_element.style(format!("width: {bar_width}%"));
    });

    view! {
        <div class="move-line move-line_time">
            <div 
                class="move-line__bar"
                node_ref=bar_ref
            />
        </div>
    }
}

#[component]
pub fn MoveLineVolume(
    volume: Signal<u8>,
) -> impl IntoView {
    const MAX_VOLUME: u8 = 100;

    let bar_ref: NodeRef<html::Div> = NodeRef::new();

    Effect::new(move |_| {
        let bar_element = bar_ref.get().unwrap();
        let bar_width = f64::trunc((volume.get() as f64 / MAX_VOLUME as f64) * 100.0) as u8;

        bar_element.style(format!("width: {bar_width}%"));
    });

    view! {
        <div class="move-line move-line_volume">
            <div 
                class="move-line__bar" 
                node_ref=bar_ref
            />
        </div>
    }
}

#[component]
pub fn MoveLineSpeed(
    initial_speed: u8,
    onchange: impl Fn(u8) + Clone + 'static,
) -> impl IntoView {
    const MAX_SPEED: u8 = 200;
    const THROTTLE_TIME: f64 = 100.0;

    let (is_mouse_down, set_is_mouse_down) = signal(false);

    let bar_initial_width = calc_bar_width(initial_speed, MAX_SPEED);
    let (bar_width, set_bar_width) = signal(bar_initial_width);

    let onchange = use_throttle_fn_with_arg(onchange, THROTTLE_TIME);
    let onchange =  Rc::new(onchange);
    let container_ref: NodeRef<html::Div> = NodeRef::new();

    Effect::new( move || {
        let container_element = container_ref.get().unwrap();
        let container_width = container_element.client_width() as u8;

        let onchange1 = onchange.clone();
        #[allow(unused)]
        use_event_listener(container_ref, ev::mousedown, move |event| {            
            set_is_mouse_down.set(true);
            
            let bar_width = event.offset_x() as u8;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange1(calc_value(initial_speed, MAX_SPEED));
        });

        let onchange2 = onchange.clone();
        #[allow(unused)]
        use_event_listener(container_ref, ev::mousemove, move |event| {
            if !is_mouse_down.get() {
                return;
            }

            let bar_width = event.offset_x() as u8;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange2(calc_value(initial_speed, MAX_SPEED));
        });

        let onchange3 = onchange.clone();
        #[allow(unused)]
        use_event_listener(container_ref, ev::mouseup, move |event| {
            if !is_mouse_down.get() {
                return;
            }

            set_is_mouse_down.set(false);

            let bar_width = event.offset_x() as u8;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange3(calc_value(initial_speed, MAX_SPEED));
        });

        #[allow(unused)]
        use_event_listener(container_ref, ev::mouseleave, move |_| {
            set_is_mouse_down.set(false);
        });
    });

    view! {
        <div 
            class="move-line move-line_speed"
            node_ref=container_ref
        >
            <div 
                class="move-line__bar"
                style:width=move || format!("{}%", bar_width.get())
            />
        </div>
    }
}

fn calc_bar_width(bar_width: u8, container_width: u8) -> u8 {
    f64::trunc((bar_width as f64 / container_width as f64) * 100.0) as u8
}

fn calc_value(bar_width: u8, container_width: u8) -> u8 {
    123
}
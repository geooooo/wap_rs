use leptos::ev;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::html;
use leptos_use::{use_event_listener, use_throttle_fn_with_arg};
use std::rc::Rc;

#[component]
pub fn MoveLineTime(
    current_time: Signal<u32>,
    duration: Signal<u32>,
    onchange: impl Fn(u32) + Clone + 'static,
) -> impl IntoView {
    view! {
        <MoveLine 
            kind=MoveLineKind::Time
            value=current_time
            max_value=duration 
            onchange
        />
    }
}

#[component]
pub fn MoveLineVolume(
    volume: Signal<u8>,
    max_volume: u8,
    onchange: impl Fn(u8) + Clone + 'static,
) -> impl IntoView {
   let volume_u32 = Signal::derive(move || {
        let val = volume.get();
        console_log(format!("ml {}", val).as_str());
        val as u32
    });
    let max_volume = Signal::derive(move || max_volume as u32);
    let onchange = move |value: u32| onchange(value as u8);

    view! {
        <MoveLine 
            kind=MoveLineKind::Volume
            value=volume_u32
            max_value=max_volume
            onchange
        />
    }
}


#[component]
pub fn MoveLineSpeed(
    initial_speed: u8,
    max_speed: u8,
    onchange: impl Fn(u8) + Clone + 'static,
) -> impl IntoView {
    let initial_speed = Signal::derive(move || initial_speed as u32);
    let max_speed = Signal::derive(move || max_speed as u32);
    let onchange = move |value: u32| onchange(value as u8);

    view! {
        <MoveLine 
            kind=MoveLineKind::Speed
            value=initial_speed
            max_value=max_speed
            onchange
        />
    }
}

enum MoveLineKind {
    Time,
    Volume,
    Speed,
}

impl MoveLineKind  {
    fn class(&self) -> &'static str {
        match self {
            Self::Time => "move-line_time",
            Self::Volume => "move-line_volume",
            Self::Speed => "move-line_speed",
        }
    }
}

#[component]
fn MoveLine(
    kind: MoveLineKind,
    value: Signal<u32>,
    max_value: Signal<u32>,
    onchange: impl Fn(u32) + Clone + 'static,
) -> impl IntoView {
    const BASE_CLASS: &str = "move-line";
    const THROTTLE_TIME: f64 = 100.0;

    let (is_mouse_down, set_is_mouse_down) = signal(false);
    let bar_width_init_fn = move || calc_bar_width(value.get(), max_value.get());
    let (bar_width, set_bar_width) = signal(bar_width_init_fn());

    let class = format!("{BASE_CLASS} {}", kind.class());

    let onchange = use_throttle_fn_with_arg(onchange, THROTTLE_TIME);
    let onchange =  Rc::new(onchange);
    let container_ref: NodeRef<html::Div> = NodeRef::new();

    Effect::new( move || {
        let has_played_track = max_value.get() != 0;
        if !has_played_track {
            return;
        }

        let container_element = container_ref.get().unwrap();
        let container_width = container_element.client_width() as u32;

        let onchange1 = onchange.clone();
   
        let _ = use_event_listener(container_ref, ev::mousedown, move |event| {            
            set_is_mouse_down.set(true);
            
            let bar_width = event.offset_x() as u32;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange1(calc_value(bar_width, container_width, max_value.get()));
        });

        let onchange2 = onchange.clone();
        let _ = use_event_listener(container_ref, ev::mousemove, move |event| {
            if !is_mouse_down.get() {
                return;
            }

            let bar_width = event.offset_x() as u32;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange2(calc_value(bar_width, container_width, max_value.get()));
        });

        let onchange3 = onchange.clone();
        let _ = use_event_listener(container_ref, ev::mouseup, move |event| {
            if !is_mouse_down.get() {
                return;
            }

            set_is_mouse_down.set(false);

            let bar_width = event.offset_x() as u32;
            set_bar_width.set(calc_bar_width(bar_width, container_width));

            onchange3(calc_value(bar_width, container_width, max_value.get()));
        });

        let _ = use_event_listener(container_ref, ev::mouseleave, move |_| {
            set_is_mouse_down.set(false);
        });
    });

    view! {
        <div 
            class=class
            node_ref=container_ref
        >
            <div 
                class="move-line__bar"
                style:width=move || format!("{}%", bar_width.get())
            />
        </div>
    }
}

fn calc_bar_width(bar_width: u32, container_width: u32) -> u32 {
    f64::trunc((bar_width as f64 / container_width as f64) * 100.0) as u32
}

fn calc_value(bar_width: u32, container_width: u32, max_value: u32) -> u32 {
    let source = bar_width as f64 / (container_width as f64 / 100.0);
    let target = (max_value as f64 / 100.0) * source;

    f64::trunc(target) as u32
}
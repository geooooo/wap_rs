use leptos::prelude::*;
use leptos::html;

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
    speed: Signal<u8>,
) -> impl IntoView {
    const MAX_SPEED: u8 = 200;

    let bar_ref: NodeRef<html::Div> = NodeRef::new();

    Effect::new(move |_| {
        let bar_element = bar_ref.get().unwrap();
        let bar_width = f64::trunc((speed.get() as f64 / MAX_SPEED as f64) * 100.0) as u8;

        bar_element.style(format!("width: {bar_width}%"));
    });

    view! {
        <div class="move-line move-line_speed">
            <div 
                class="move-line__bar"
                node_ref=bar_ref
            />
        </div>
    }
}
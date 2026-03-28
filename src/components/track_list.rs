use web_sys as web;
use leptos::web_sys::HtmlElement;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, html, prelude::*};
use leptos_use::{UseEventListenerOptions, use_event_listener, use_event_listener_with_options};
use crate::state::{AppState, TrackUiState};

#[component]
pub fn TrackList(
    is_hidden: Signal<bool>,
    tracks: Signal<Vec<TrackUiState>>,
    onclick: impl Fn(String, bool, bool) + Clone + 'static,
    mut onfilesdrop: impl FnMut(web::FileList) + 'static,
) -> impl IntoView {
    let container_ref: NodeRef<html::Div> = NodeRef::new();
    let drop_zone_ref: NodeRef<html::Div> = NodeRef::new();

    let _ = use_event_listener_with_options(drop_zone_ref, ev::dragenter, move |event| {          
        event.prevent_default();
        event.stop_propagation();
    }, UseEventListenerOptions::default().capture(false));

    let _ = use_event_listener_with_options(drop_zone_ref, ev::dragover, move |event| {          
        event.prevent_default();
        event.stop_propagation();
    }, UseEventListenerOptions::default().capture(false));

    let _ = use_event_listener_with_options(drop_zone_ref, ev::drag, move |event| {          
        event.prevent_default();
        event.stop_propagation();
    }, UseEventListenerOptions::default().capture(false));

    let _ = use_event_listener_with_options(drop_zone_ref, ev::dragleave, move |event| {          
        event.prevent_default();
        event.stop_propagation();
    }, UseEventListenerOptions::default().capture(false));

    let _ = use_event_listener_with_options(drop_zone_ref, ev::drop, move |event| {          
        event.prevent_default();
        event.stop_propagation();

        let files = event.data_transfer().unwrap().files().unwrap();
        onfilesdrop(files);
    }, UseEventListenerOptions::default().capture(false));

    let _ = use_event_listener(container_ref, ev::mouseup, move |event| {          
        if let Some(target) = event.target() && let Ok(mut element) = target.dyn_into::<HtmlElement>() {
            if !element.class_list().contains("track") {
                element = element.parent_element().unwrap().dyn_into::<HtmlElement>().unwrap();
            }

            let track_name = element.query_selector(".track__title").unwrap().unwrap().text_content().unwrap();

            if event.meta_key() {
                if element.class_list().contains("track_played") {    
                    return;
                }
                
                element.class_list().toggle("track_selected").unwrap();
                let is_selected = element.class_list().contains("track_selected");
                
                onclick(track_name, is_selected, false);
            } else {
                element.class_list().add_1("track_played").unwrap();

                let is_selected = element.class_list().contains("track_selected");
                let is_played = element.class_list().contains("track_played");   
                onclick(track_name, is_selected, is_played);
            }
        }
    });
    
    view! {
        <div 
            node_ref=drop_zone_ref
            class="track-list"
            class:track-list_hidden=is_hidden
        >
            <Show
                when=move || !tracks.get().is_empty()
                fallback=|| view! {
                    <div class="track-list__empty-text">
                        Move your tracks here
                    </div>
                }
            >
                <div 
                    node_ref=container_ref
                    class="track-list__container">
                    <ForEnumerate
                        each=move || tracks.get()
                        key=|track| track.id()
                        children={move |_, track: TrackUiState| view! {
                            <Track 
                                name=track.name 
                                duration=track.duration
                                is_selected=track.is_selected
                                is_played=track.is_played
                            />
                        }}
                    />
                </div>
            </Show>
        </div>
    }
}

#[component]
fn Track(
    name: String,
    duration: u32,
    is_selected: bool,
    is_played: bool,
) -> impl IntoView {
    let formatted_duration = AppState::format_time(duration);

    view! {
        <div 
            class:track_selected=is_selected
            class:track_played=is_played
            class="track"
        >
            <div class="track__title">
                {name}
            </div>

            <div class="track__time">
                {formatted_duration}
            </div>
        </div>
    }
}
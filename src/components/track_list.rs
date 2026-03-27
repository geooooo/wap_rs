use leptos::web_sys::HtmlElement;
use leptos::wasm_bindgen::JsCast;
use leptos::{ev, html, prelude::*};
use leptos_use::use_event_listener;
use crate::state::{AppState, Track as TrackModel};

#[component]
pub fn TrackList(
    is_hidden: Signal<bool>,
    tracks: Signal<Vec<TrackModel>>,
    onclick: impl Fn(String, bool, bool) + Clone + 'static,
) -> impl IntoView {
    let container_ref: NodeRef<html::Div> = NodeRef::new();

    let _ = use_event_listener(container_ref, ev::mouseup, move |event| {          
        if let Some(target) = event.target() && let Ok(mut element) = target.dyn_into::<HtmlElement>() {
            if !element.class_list().contains("track") {
                element = element.parent_element().unwrap().dyn_into::<HtmlElement>().unwrap();
            }

            let track_name = element.query_selector(".track__title").unwrap().unwrap().text_content().unwrap();

            if event.meta_key() {
                element.class_list().toggle("track_selected").unwrap();
                
                let is_selected = element.class_list().contains("track_selected");
                let is_played = element.class_list().contains("track_played");    
                onclick(track_name, is_selected, is_played);
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
                        key=|track| format!("{}{}{}", track.name, track.is_played, track.is_selected)
                        children={move |_, track: TrackModel| view! {
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
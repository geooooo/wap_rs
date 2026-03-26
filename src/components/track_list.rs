use leptos::prelude::*;
use crate::state::{AppState, Track as TrackModel};

// TODO: click on track - select as played but not play now and pause
// shift+lclick select range of tracks
// cmd+lclick select single track
// cmd+a select all tracks
// "ArrowUp" | "ArrowLeft" => return leptos::logging::debug_log!("up"),
//             "ArrowDown" | "ArrowRight" => return leptos::logging::debug_log!("down"),
//             "Backspace" => return leptos::logging::debug_log!("back"),
//             "Escape" => return leptos::logging::debug_log!("esc"),
#[component]
pub fn TrackList(
    is_hidden: Signal<bool>,
    tracks: Signal<Vec<TrackModel>>,
) -> impl IntoView {
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
                <div class="track-list__container">
                    <ForEnumerate
                        each=move || tracks.get()
                        key=|track| track.name.clone()
                        children={move |_, track: TrackModel| view! {
                            <Track 
                                name=track.name 
                                duration=track.duration
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
) -> impl IntoView {
    let formatted_duration = AppState::format_time(duration);

    view! {
        <div class="track">
            <div class="track__title">
                {name}
            </div>

            <div class="track__time">
                {formatted_duration}
            </div>
        </div>
    }
}
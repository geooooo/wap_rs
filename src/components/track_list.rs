use leptos::prelude::*;
use crate::state::{AppState, Track as TrackModel};

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
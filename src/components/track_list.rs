use leptos::prelude::*;
use crate::state::{AppState, Track as TrackModel};

// TODO: click on track - select as played but not play now and pause
// shift+lclick select range of tracks
// cmd+lclick select single track
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
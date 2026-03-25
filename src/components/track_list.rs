use leptos::prelude::*;
use crate::state::Track as TrackModel;

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
    let hours = duration / 3600;
    let minutes = duration % 3600 / 60;
    let seconds = duration % 60;

    let formatted_duration = if hours > 0 {
        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    } else {
        format!("{minutes:0>2}:{seconds:0>2}")
    };

    view! {
        <div class="track">
            <div class="track__title">
                {name}
            </div>

            <div class="track__time">
                {duration} " - " {formatted_duration}
            </div>
        </div>
    }
}
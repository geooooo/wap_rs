use leptos::prelude::*;

#[component]
pub fn TrackList() -> impl IntoView {
    view! {
        <div class="track-list track-list_hidden_">
            <div class="track-list__empty-text">Move your tracks here</div>

            <div class="track-list__container">
                <Track />
            </div>
        </div>
    }
}

#[component]
fn Track() -> impl IntoView {
    view! {
        <div class="track">
            <div class="track__title"></div>
            <div class="track__time"></div>
        </div>
    }
}
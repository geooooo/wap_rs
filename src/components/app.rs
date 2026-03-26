use leptos::prelude::*;
use super::equalizer::Equalizer;
use super::move_line::{MoveLineTime, MoveLineVolume, MoveLineSpeed};
use super::flat_button::{
    PrevFlatButton, 
    PlayFlatButton, 
    NextFlatButton,
    ListFlatButton,
    RandomFlatButton,
    LoopFlatButton,
};
use super::logo::Logo;
use super::track_list::TrackList;
use crate::state::AppState;

// TODO: move line
// TODO: helper text: animation, speed, volume, time, 3 pressed buttons
// TODO: hot keys: space - play/pause, enter - toggle list, arrows to select track, r - random, l - loop, p - pause/play, +/- - volume
// TODO: backspace - delete track, shift+lclick select range of tracks, cmd+lclick select single track
// TODO: cmd+a select all tracks, esc clear selection, click on track - select as played but not play now and pause
#[component]
pub fn App() -> impl IntoView {
    let (state, set_state) = signal(AppState::default());

    let help_text = 
        Signal::derive(move || state.get().get_help_text());

    let duration = 
        Signal::derive(move || state.get().get_track_duration());

    let current_time = 
        Signal::derive(move || state.get().get_time());

    let volume = 
        Signal::derive(move || state.get().get_volume());

    let play_state = 
        Signal::derive(move || state.get().get_play_state());

    let is_track_list_hidden = 
        Signal::derive(move || !state.get().is_track_list_visible());

    let tracks = 
        Signal::derive(move || state.get().get_tracks());

    let on_play_button_click = move || 
        set_state.update(|state| state.toggle_play_state());

    let on_random_button_click = move || 
        set_state.update(|state| state.toggle_random());

    let on_loop_button_click = move || 
        set_state.update(|state| state.toggle_loop());

    let on_list_button_click = move || 
        set_state.update(|state| state.toggle_track_list_visibility());

    let on_prev_button_click = move || {};

    let on_next_button_click = move || {};

    view! {
        <div class="app__controls">
            <div class="controls__top">
                <Equalizer help_text />
                <MoveLineTime 
                    duration
                    current_time
                />
            </div>

            <div class="controls__bottom">
                <div class="controls__col">
                    <MoveLineVolume 
                        volume
                    />
                    <MoveLineSpeed
                        initial_speed=state.get_untracked().get_speed()
                        onchange=|v| leptos::logging::debug_log!("{v}")
                    />
                </div>

                <div class="controls__row">
                    <PrevFlatButton onclick=on_prev_button_click />

                    <PlayFlatButton 
                        play_state=play_state
                        onclick=on_play_button_click
                    />

                    <NextFlatButton onclick=on_next_button_click />
                </div>

                <div class="controls__row">
                    <ListFlatButton onclick=on_list_button_click />
                    <RandomFlatButton onclick=on_random_button_click />
                    <LoopFlatButton onclick=on_loop_button_click />
                </div>
            </div>
        </div>

        <div class="app__logo">
            <Logo />
        </div>

        <div class="app__track-list">
            <TrackList 
                is_hidden=is_track_list_hidden
                tracks
            />
        </div>
    }
}
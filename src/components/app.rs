use leptos::prelude::*;
use super::move_line::*;
use super::flat_button::*;
use super::equalizer::Equalizer;
use super::logo::Logo;
use super::track_list::TrackList;
use crate::state::{AppState, HelpTarget};

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

    let on_prev_button_click = || {};

    let on_next_button_click = || {};

    let on_volume_change = move |volume|
        set_state.update(|state| state.set_volume(volume));

    let on_speed_change = move |speed|
        set_state.update(|state| state.set_speed(speed));

    let on_current_time_change = move |time|
        set_state.update(|state| state.set_current_time(time));

    let on_volume_hover = move |_|
        set_state.update(|state| state.update_help_text(HelpTarget::VolumeLine));

    let on_speed_hover = move |_|
        set_state.update(|state| state.update_help_text(HelpTarget::SpeedLine));

    let on_current_time_hover = move |_| 
        set_state.update(|state| state.update_help_text(HelpTarget::TimeLine));

    let on_loop_button_hover = move |_| 
        set_state.update(|state| state.update_help_text(HelpTarget::LoopButton));

    let on_random_button_hover = move |_| 
        set_state.update(|state| state.update_help_text(HelpTarget::RandomButton));

    let on_list_button_hover = move |_|
        set_state.update(|state| state.update_help_text(HelpTarget::ListButton));

    view! {
        <div class="app__controls">
            <div class="controls__top">
                <Equalizer help_text />
                <MoveLineTime 
                    current_time
                    duration
                    onchange=on_current_time_change
                    on:mouseenter=on_current_time_hover
                />
            </div>

            <div class="controls__bottom">
                <div class="controls__col">
                    <MoveLineVolume 
                        initial_volume=state.get_untracked().get_volume()
                        max_volume=AppState::MAX_VOLUME
                        onchange=on_volume_change
                        on:mouseenter=on_volume_hover
                    />
                    <MoveLineSpeed
                        initial_speed=state.get_untracked().get_speed()
                        max_speed=AppState::MAX_SPEED
                        onchange=on_speed_change
                        on:mouseenter=on_speed_hover
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
                    <ListFlatButton 
                        onclick=on_list_button_click
                        on:mouseenter=on_list_button_hover
                    />

                    <RandomFlatButton 
                        onclick=on_random_button_click
                        on:mouseenter=on_random_button_hover
                    />

                    <LoopFlatButton 
                        onclick=on_loop_button_click
                        on:mouseenter=on_loop_button_hover
                    />
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
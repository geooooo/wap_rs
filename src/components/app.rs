use leptos::prelude::*;
use leptos::html;
use super::move_line::*;
use super::flat_button::*;
use super::equalizer::Equalizer;
use super::logo::Logo;
use super::track_list::TrackList;
use crate::state::{AppState, HelpTarget, PlayState};
use crate::web::init_global_key_event_handlers;

#[component]
pub fn App() -> impl IntoView {
    let random_button_ref: NodeRef<html::Button> = NodeRef::new();
    let loop_button_ref: NodeRef<html::Button> = NodeRef::new();
    let list_button_ref: NodeRef<html::Button> = NodeRef::new();


    let state = RwSignal::new(AppState::default());

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


    let on_prev_button_click = move ||
        state.update(|state| {
            state.set_prev_track();
            state.set_play_state();
            
            match state.get_track() {
                None => (),
                Some(_) => (), // play
            }
        });

    let on_next_button_click = move ||
        state.update(|state| {
            state.set_next_track();
            state.set_play_state();
            
            match state.get_track() {
                None => (),
                Some(_) => (), // play
            }
        });

    let on_play_button_click = move || 
        state.update(|state| {
            state.toggle_play_state();

            match state.get_play_state() {
                PlayState::NoTrack => (),
                PlayState::Play => (), //play
                PlayState::Pause => (), //pause
            }
        });

    let on_random_button_click = move ||
        state.update(|state| state.toggle_random());

    let on_loop_button_click = move || 
        state.update(|state| state.toggle_loop());

    let on_list_button_click = move || 
        state.update(|state| state.toggle_track_list_visibility());


    let on_volume_change = move |volume|
        state.update(|state| state.set_volume(volume as u8));

    let on_speed_change = move |speed|
        state.update(|state| state.set_speed(speed as u8));

    let on_current_time_change = move |time|
        state.update(|state| state.set_current_time(time));


    let on_volume_hover = move |_|
        state.update(|state| state.set_help_text(HelpTarget::VolumeLine));

    let on_speed_hover = move |_|
        state.update(|state| state.set_help_text(HelpTarget::SpeedLine));

    let on_current_time_hover = move |_| 
        state.update(|state| state.set_help_text(HelpTarget::TimeLine));

    let on_loop_button_hover = move |_| 
        state.update(|state| state.set_help_text(HelpTarget::LoopButton));

    let on_random_button_hover = move |_| 
        state.update(|state| state.set_help_text(HelpTarget::RandomButton));

    let on_list_button_hover = move |_|
        state.update(|state| state.set_help_text(HelpTarget::ListButton));

    
    let random_button_click = move || 
        random_button_ref.get().unwrap().click();

    let loop_button_click = move || 
        loop_button_ref.get().unwrap().click();

    let list_button_click = move || 
        list_button_ref.get().unwrap().click();

    let change_volume = move |is_inc| 
        if is_inc {
            state.update(|state| state.inc_volume());
        } else {
            state.update(|state| state.dec_volume());
        };

    let remove_selected_tracks = || {};

    let deselect_all_tracks = || {};

    let select_all_tracks = || {};


    init_global_key_event_handlers(
        random_button_click,
        loop_button_click,
        list_button_click,
        on_play_button_click,
        change_volume,
        on_next_button_click,
        on_prev_button_click,
        remove_selected_tracks,
        deselect_all_tracks,
        select_all_tracks,
    );


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
                        volume
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
                        node_ref=list_button_ref
                        onclick=on_list_button_click
                        on:mouseenter=on_list_button_hover
                    />

                    <RandomFlatButton 
                        node_ref=random_button_ref
                        onclick=on_random_button_click
                        on:mouseenter=on_random_button_hover
                    />

                    <LoopFlatButton 
                        node_ref=loop_button_ref
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
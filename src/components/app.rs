use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::html;
use super::move_line::*;
use super::flat_button::*;
use super::equalizer::Equalizer;
use super::logo::Logo;
use super::track_list::TrackList;
use crate::state::{AppState, HelpTarget};
use crate::web::init_global_key_event_handlers;

#[component]
pub fn App() -> impl IntoView {
    let random_button_ref: NodeRef<html::Button> = NodeRef::new();
    let loop_button_ref: NodeRef<html::Button> = NodeRef::new();
    let list_button_ref: NodeRef<html::Button> = NodeRef::new();


    let (state, set_state) = signal(AppState::default());

    let help_text = 
        Signal::derive(move || state.get().get_help_text());

    let duration = 
        Signal::derive(move || state.get().get_track_duration());

    let current_time = 
        Signal::derive(move || state.get().get_time());

    let volume = 
        Signal::derive(move || {
            console_log(format!("app {}", state.get().get_volume()).as_str());
            state.get().get_volume()
        });

    let play_state = 
        Signal::derive(move || state.get().get_play_state());

    let is_track_list_hidden = 
        Signal::derive(move || !state.get().is_track_list_visible());

    let tracks = 
        Signal::derive(move || state.get().get_tracks());


    let on_prev_button_click = |_| {};

    let on_next_button_click = |_| {};

    let on_play_button_click = move |_| 
        set_state.update(|state| state.toggle_play_state());

    let on_random_button_click = move |is_user|
        set_state.update(|state| {
            state.toggle_random();

            if is_user {
                state.update_help_text(HelpTarget::RandomButton);
            } else {
                state.hide_help_text();
            }
        });

    let on_loop_button_click = move |is_user| 
        set_state.update(|state| {
            state.toggle_loop();

            if is_user {
                state.update_help_text(HelpTarget::LoopButton);
            } else {
                state.hide_help_text();
            }
        });

    let on_list_button_click = move |is_user| 
        set_state.update(|state| {
            state.toggle_track_list_visibility();

            if is_user {
                state.update_help_text(HelpTarget::ListButton);
            } else {
                state.hide_help_text();
            }
        });

    let random_button_click = move ||
        random_button_ref.get().unwrap().click();

    let loop_button_click = move || 
        loop_button_ref.get().unwrap().click();

    let list_button_click = move || 
        list_button_ref.get().unwrap().click();

    let change_volume = move |is_inc| if is_inc {
        set_state.update(|state| state.inc_volume())
    } else {
        set_state.update(|state| state.dec_volume())
    };

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


    init_global_key_event_handlers(
        random_button_click,
        loop_button_click,
        list_button_click,
        move || on_play_button_click(false),
        change_volume,
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
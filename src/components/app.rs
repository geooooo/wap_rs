use std::sync::Arc;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::html;
use wasm_bindgen_futures::spawn_local;
use super::move_line::*;
use super::flat_button::*;
use super::equalizer::Equalizer;
use super::logo::Logo;
use super::track_list::TrackList;
use crate::state::{AppState, HelpTarget, PlayState};
use crate::web::{init_global_key_event_handlers, Player};

#[component]
pub fn App() -> impl IntoView {
    let random_button_ref: NodeRef<html::Button> = NodeRef::new();
    let loop_button_ref: NodeRef<html::Button> = NodeRef::new();
    let list_button_ref: NodeRef<html::Button> = NodeRef::new();

    let state = RwSignal::new(AppState::default());

    let player = Arc::new(Player::new(
        state.with_untracked(|state| state.get_volume()),
        state.with_untracked(|state| state.get_speed()),
        move |time| state.update(|state| state.set_time(time)),
    ));

    let help_text = create_read_slice(
        state,
        |state| state.get_help_text(),
    );

    let duration = create_read_slice(
        state,
        |state| state.get_track_duration(),
    );

    let current_time = create_read_slice(
        state,
        |state| state.get_time(),
    );

    let max_volume = create_read_slice(
        state,
        |_| AppState::MAX_VOLUME as u32,
    );

    let volume = create_read_slice(
        state,
        |state| state.get_volume() as u32,
    );

    let max_speed = create_read_slice(
        state,
        |_| AppState::MAX_SPEED as u32,
    );

    let initial_speed = create_read_slice(
        state,
        |state| state.get_speed() as u32,
    );

    let play_state = create_read_slice(
        state,
        |state| state.get_play_state(),
    );

    let is_track_list_hidden = create_read_slice(
        state,
        |state| !state.is_track_list_visible(),
    );

    let tracks = Signal::derive(move || 
        state.with(|state| state.get_tracks())
    );

    let player1 = player.clone();
    let on_prev_button_click = Arc::new(move ||
        state.update(|state| {
            state.set_prev_track();
            state.set_play_state();
            state.set_time(0);

            match state.get_track() {
                None => (),
                Some(data) => player1.play_track(data),
            } 
        }));

    let player2 = player.clone();
    let on_next_button_click = Arc::new(move || 
        state.update(|state| {
            state.set_next_track();
            state.set_play_state();
            state.set_time(0);
            
            match state.get_track() {
                None => (),
                Some(data) => player2.play_track(data),
            }
        }));

    let player3 = player.clone();
    let on_play_button_click = Arc::new(move || {
        state.update(|state| {
            let (prev, current) = state.toggle_play_state();

            match current {
                PlayState::NoTrack => (),
                PlayState::Pause => player3.pause(),
                PlayState::Play => match prev {
                    PlayState::NoTrack => {
                        match state.get_track() {
                            None => (),
                            Some(data) => {
                                state.set_time(0);
                                player3.play_track(data);
                            },
                        }
                    },
                    _ => player3.play(),
                },
            }
        });
    });

    let player4 = player.clone();
    let on_track_list_click = move |track_name, is_selected, is_played|
        state.update(|state| { 
            state.update_track_state(track_name, is_selected, is_played);
            state.set_play_state();

            if !is_played {
                return;
            }

            match state.get_track() {
                None => (),
                Some(data) => {
                    state.set_time(0);
                    player4.play_track(data);
                },
            }
        });

    let on_random_button_click = move ||
        state.update(|state| state.toggle_random());

    let player5 = player.clone();
    let on_loop_button_click = move || 
        state.update(|state| {
            state.toggle_loop();

            player5.set_loop(state.is_loop());
        });

    let on_list_button_click = move || 
        state.update(|state| state.toggle_track_list_visibility());

    let player6 = player.clone();
    let on_files_drop = move |files| {
        let player6 = player6.clone();

        spawn_local(async move {
            let current_tracks = state.with_untracked(|s| s.get_tracks());
            let new_tracks = player6.parse_files(files, current_tracks).await;

            state.update(|state| state.update_tracks(new_tracks));
        });
    };
        

    let player7 = player.clone();
    let on_volume_change = move |volume|
        state.update(|state| {
            state.set_volume(volume as u8);

            player7.set_volume(volume as u8);
        });

    let player8 = player.clone();
    let on_speed_change = move |speed|
        state.update(|state| {
            state.set_speed(speed as u8);

            player8.set_speed(speed as u8);
        });

    let player9 = player.clone();
    let on_current_time_change = move |time|
        state.update(|state| {
            state.set_time(time);
            console_log(format!("{}", time).as_str());

            player9.set_time(time);
        });


    let on_volume_hover = move |_|
        state.update(|state| state.set_help_text(HelpTarget::VolumeLine));

    let on_speed_hover = move |_|
        state.update(|state| state.set_help_text(HelpTarget::SpeedLine));

    let on_current_time_hover = move |_| 
        state.update(|state| {
            state.set_is_time_hovered(true);
            state.set_help_text(HelpTarget::TimeLine);
        });

    let on_current_time_not_hover = move |_| 
        state.update(|state| {
            state.set_is_time_hovered(false);
            state.set_help_text(HelpTarget::TimeLine);
        });

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

    let player10 = player.clone();
    let change_volume = move |is_inc| 
        state.update(|state| {
            if is_inc {
                state.inc_volume();
            } else {
                state.dec_volume()
            }

            player10.set_volume(state.get_volume());
        });

    let remove_selected_tracks = move ||
        state.update(|state| state.remove_selected_tracks());

    let deselect_all_tracks = move ||
        state.update(|state| state.deselect_all_tracks());

    let select_all_tracks = move ||
        state.update(|state| state.select_all_tracks());


    init_global_key_event_handlers(
        random_button_click,
        loop_button_click,
        list_button_click,
        { 
            let handler = on_play_button_click.clone(); 
            move || handler() 
        },
        change_volume,
        { 
            let handler = on_next_button_click.clone(); 
            move || handler() 
        },
        { 
            let handler = on_prev_button_click.clone(); 
            move || handler() 
        },
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
                    on:mouseleave=on_current_time_not_hover
                />
            </div>

            <div class="controls__bottom">
                <div class="controls__col">
                    <MoveLineVolume 
                        volume
                        max_volume
                        onchange=on_volume_change
                        on:mouseenter=on_volume_hover
                    />
                    <MoveLineSpeed
                        initial_speed
                        max_speed
                        onchange=on_speed_change
                        on:mouseenter=on_speed_hover
                    />
                </div>

                <div class="controls__row">
                    <PrevFlatButton onclick=move || on_prev_button_click() />

                    <PlayFlatButton 
                        play_state=play_state
                        onclick=move || on_play_button_click()
                    />

                    <NextFlatButton onclick=move || on_next_button_click() />
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
                onclick=on_track_list_click
                onfilesdrop=on_files_drop
            />
        </div>
    }
}
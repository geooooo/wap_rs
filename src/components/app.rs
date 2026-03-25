use leptos::prelude::*;
use super::equalizer::Equalizer;
use super::move_line::{MoveLine, MoveLineKind};
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

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app__controls">
            <div class="controls__top">
                <Equalizer />
                <MoveLine kind=MoveLineKind::Time/>
            </div>

            <div class="controls__bottom">
                <div class="controls__col">
                    <MoveLine kind=MoveLineKind::Volume/>
                    <MoveLine kind=MoveLineKind::Speed/>
                </div>

                <div class="controls__row">
                    <PrevFlatButton />
                    <PlayFlatButton />
                    <NextFlatButton />
                </div>

                <div class="controls__row">
                    <ListFlatButton />
                    <RandomFlatButton />
                    <LoopFlatButton />
                </div>
            </div>
        </div>

        <div class="app__logo">
            <Logo />
        </div>

        <div class="app__track-list">
            <TrackList />
        </div>
    }
}
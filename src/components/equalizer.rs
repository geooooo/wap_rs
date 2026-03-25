use leptos::prelude::*;
use super::helper_text::HelperText;

#[component]
pub fn Equalizer() -> impl IntoView {
    view! {
        <div class="equalizer">
            <div class="equalizer__container">
                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>

                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>
                <div class="equalizer__level"></div>
            </div>

            <HelperText />
        </div>
    }
}
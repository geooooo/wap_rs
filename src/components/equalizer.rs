use leptos::prelude::*;
use super::helper_text::HelperText;

#[component]
pub fn Equalizer(
    help_text: Signal<String>,
) -> impl IntoView {
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

            <HelperText text=help_text />
        </div>
    }
}
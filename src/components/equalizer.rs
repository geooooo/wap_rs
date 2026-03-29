use leptos::prelude::*;
use super::helper_text::HelperText;

#[component]
pub fn Equalizer(
    levels: Signal<Vec<u8>>,
    help_text: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="equalizer">
            <div class="equalizer__container">
                {move || levels.get()
                    .into_iter()
                    .map(|level| {
                        let level_percent = (level as f64 / u8::MAX as f64 * 100.0) as u8;
                        view! {
                          <div 
                            style:height=format!("{}%", level_percent)
                            class="equalizer__level" />
                        }
                    })
                    .collect_view()
                }
            </div>

            <HelperText text=help_text />
        </div>
    }
}
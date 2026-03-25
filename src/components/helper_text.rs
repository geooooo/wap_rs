use leptos::prelude::*;

#[component]
pub fn HelperText(
    text: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="helper-text">
            {move || text.get()}
        </div>
    }
}
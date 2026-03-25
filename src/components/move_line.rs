use leptos::prelude::*;

pub enum MoveLineKind {
    Time,
    Volume,
    Speed,
}

#[component]
pub fn MoveLine(
    kind: MoveLineKind,
) -> impl IntoView {
    let kind = match kind {
        MoveLineKind::Time => "move-line move-line_time",
        MoveLineKind::Volume => "move-line move-line_volume",
        MoveLineKind::Speed => "move-line move-line_speed",
    };

    view! {
        <div class=kind>
            <div class="move-line__bar"></div>
        </div>
    }
}